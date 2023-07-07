package route

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/fatih/color"
	"github.com/ghostff/payment-frame/app/models"
	"github.com/go-playground/locales/en"
	ut "github.com/go-playground/universal-translator"
	"github.com/go-playground/validator/v10"
	entranslations "github.com/go-playground/validator/v10/translations/en"
	"github.com/rs/cors"
	"html/template"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"reflect"
	"regexp"
	"runtime"
	"strings"
)

type GuardResponse struct {
	valid   bool
	message string
	code    int
}
type GuardCallbackType = func(*ControlScope, *ControllerArg) GuardResponse
type GuardType []GuardCallbackType
type RequestCallbackType func(*ControllerArg)

type ControlScope struct {
	cors       *cors.Cors
	guards     map[string]func(*ControlScope, *ControllerArg) GuardResponse
	controller RequestCallbackType
	name       string
}

type requestTypeMap map[string]*ControlScope

type Router struct {
	namespace      string
	guards         map[string]GuardCallbackType
	post           requestTypeMap
	put            requestTypeMap
	delete         requestTypeMap
	get            requestTypeMap
	options        requestTypeMap
	routeRegexps   []string
	fastRouteRegex string
	activeCors     *cors.Cors
}

type ControllerArg struct {
	Response    http.ResponseWriter
	Request     *http.Request
	UrlParams   map[string]string
	QueryParams map[string]string
	layout      string
}

type BREAD interface {
	Browse(*ControllerArg)
	Read(*ControllerArg)
	Edit(*ControllerArg)
	Add(*ControllerArg)
	Delete(*ControllerArg)
}

var (
	templates  *template.Template
	validate   *validator.Validate
	translator ut.Translator
)

func (r *Router) addRoute(method string, uri string, handler RequestCallbackType, name []string) {
	uri = strings.Trim(r.namespace+uri, "/")
	guards := r.guards
	if len(r.guards) == 0 {
		guards = nil
	}

	routeScope := &ControlScope{
		cors:       r.activeCors,
		guards:     guards,
		controller: handler,
		name:       strings.Join(name, ""),
	}

	if strings.ContainsRune(uri, '{') {
		regex := regexp.MustCompile(`/\{(.*?)(?::(.*?))?}(\?)?`)
		route := uri
		routeName := fmt.Sprintf("r%d", len(r.routeRegexps))

		for _, match := range regex.FindAllStringSubmatch(uri, -1) {
			routePattern := "(?:/(?P<%s_%s>%s))%s"
			// If route does not have regex
			if match[2] == "" {
				routePattern = "(?:/(?P<%s_%s>[^/]+))%s%s"
			}
			route = strings.ReplaceAll(route, match[0], fmt.Sprintf(routePattern, routeName, match[1], match[2], match[3]))
		}

		r.routeRegexps = append(r.routeRegexps, fmt.Sprintf("(?P<%s>^%s$)", routeName, route))
		uri = routeName
	}

	if r.activeCors != nil {
		r.options[uri] = routeScope
		r.activeCors = nil
	}

	switch method {
	case http.MethodGet:
		r.get[uri] = routeScope
	case http.MethodPost:
		r.post[uri] = routeScope
	case http.MethodPut:
		r.put[uri] = routeScope
	case http.MethodDelete:
		r.delete[uri] = routeScope
	}
}

func (r *Router) getType(method string) (requestTypeMap, error) {
	switch method {
	case http.MethodGet:
		return r.get, nil
	case http.MethodPost:
		return r.post, nil
	case http.MethodPut:
		return r.put, nil
	case http.MethodDelete:
		return r.delete, nil
	case http.MethodOptions:
		return r.options, nil
	default:
		return nil, errors.New("Method not allowed")
	}
}

func (r *Router) Bread(uri string, handler BREAD, name ...string) {
	if len(name) == 0 {
		name = append(name, "")
	}

	uriWith := uri + "/{id}"
	r.addRoute(http.MethodGet, uri, handler.Browse, []string{name[0] + ".browse"})
	r.addRoute(http.MethodGet, uriWith, handler.Read, []string{name[0] + ".get"})
	r.addRoute(http.MethodPut, uriWith, handler.Edit, []string{name[0] + ".get"})
	r.addRoute(http.MethodPost, uri, handler.Add, []string{name[0] + ".edit"})
	r.addRoute(http.MethodDelete, uriWith, handler.Delete, []string{name[0] + ".delete"})
}

func (r *Router) Get(uri string, handler RequestCallbackType, name ...string) {
	r.addRoute(http.MethodGet, uri, handler, name)
}

func (r *Router) Post(uri string, handler RequestCallbackType, name ...string) {
	r.addRoute(http.MethodPost, uri, handler, name)
}

func (r *Router) Put(uri string, handler RequestCallbackType, name ...string) {
	r.addRoute(http.MethodPut, uri, handler, name)
}

func (r *Router) Delete(uri string, handler RequestCallbackType, name ...string) {
	r.addRoute(http.MethodDelete, uri, handler, name)
}

func (r *Router) Cors(cors *cors.Cors) *Router {
	r.activeCors = cors

	return r
}

func (r *Router) Group(namespace string, guards GuardType, callback func(*Router)) {
	if strings.TrimSpace(namespace) != "" {
		r.namespace = "/" + strings.Trim(r.namespace+namespace, "/") + "/"
	}

	if guards != nil {
		for _, fn := range guards {
			r.guards[getFunctionName(fn)] = fn // map is being used to prevent duplicates
		}
	}

	// this calls the routes in current group
	callback(r)
	r.initializeGroup()
	r.activeCors = nil
}

func (r *Router) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	logRequest(req)

	defer func() {
		if err := recover(); err != nil {
			fmt.Println("Recovered:", err)

			// Return a 500 Internal Server Error
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		}
	}()

	// Check if the request path matches the static file prefix
	if strings.HasPrefix(req.URL.Path, "/assets") {
		// Serve static files directly
		r.serveStaticFiles(w, req)
		return
	}

	// get route list base on request method
	route, err := r.getType(req.Method)
	if err != nil {
		http.Error(w, err.Error(), http.StatusMethodNotAllowed)
		return
	}

	uri := strings.Trim(req.URL.Path, "/")
	requestParams := map[string]string{}

	// try to find route by static name
	handler, found := route[uri]
	if !found {
		matchName := ""
		if r.fastRouteRegex != "" {
			regex := regexp.MustCompile(r.fastRouteRegex)
			names := regex.SubexpNames()

			for i, name := range regex.FindStringSubmatch(uri) {
				if i != 0 && name != "" {
					if matchName == "" {
						matchName = names[i]
						continue
					}

					requestParams[strings.TrimPrefix(names[i], matchName+"_")] = name
				}
			}
		}

		if matchName == "" {
			http.NotFound(w, req)
			color.Red("404 for %s(%s) make sure route request method is valid", req.Method, uri)
			return
		}

		handler, _ = route[matchName]
	}

	if handler.cors != nil { // Handle preflight
		// Set headers
		handler.cors.HandlerFunc(w, req)
		// Dont continue if option.
		if req.Method == http.MethodOptions {
			return
		}
	}

	arg := &ControllerArg{
		Request:     req,
		Response:    w,
		UrlParams:   requestParams,
		QueryParams: map[string]string{},
		layout:      "layout.gohtml",
	}

	// Evaluate guards
	for _, guardCallback := range handler.guards {
		guard := guardCallback(handler, arg)
		if guard.valid == false {
			http.Error(w, guard.message, http.StatusUnauthorized)
			return
		}
	}

	// Change content type if request is json
	// We're doing this here to allow overrides in controller
	if strings.HasPrefix(req.Header.Get("Accept"), "application/json") {
		w.Header().Set("Content-Type", "application/json; charset=utf-8")
	}

	handler.controller(arg)
}

func (r *Router) serveStaticFiles(w http.ResponseWriter, req *http.Request) {
	// Get the requested file path
	filePath := filepath.Join("public", req.URL.Path)

	// Check if the file exists
	_, err := os.Stat(filePath)
	if os.IsNotExist(err) {
		// File not found, return a 404 response
		http.NotFound(w, req)
		return
	}

	// Serve the static file
	http.ServeFile(w, req, filePath)
}

func IsJSONRequest(_ *ControlScope, arg *ControllerArg) GuardResponse {
	contentType := arg.Request.Header.Get("Content-Type")

	return GuardResponse{
		valid:   strings.HasPrefix(contentType, "application/json"),
		message: fmt.Sprintf("Unsupported content type '%s'. application/json expected.", contentType),
		code:    http.StatusUnsupportedMediaType,
	}
}

func Boostrap(routes ...func(*Router)) {
	router := &Router{
		post:           requestTypeMap{},
		put:            requestTypeMap{},
		delete:         requestTypeMap{},
		get:            requestTypeMap{},
		options:        requestTypeMap{},
		fastRouteRegex: "",
		routeRegexps:   []string{},
	}

	// check if request was canceled and exit

	router.initializeGroup()
	for _, fn := range routes {
		fn(router)
	}

	if len(router.routeRegexps) > 0 {
		router.fastRouteRegex = strings.Join(router.routeRegexps, "|")
		router.routeRegexps = nil
	}

	english := en.New()
	uni := ut.New(english, english)
	translator, _ = uni.GetTranslator("en")

	validate = validator.New()
	_ = entranslations.RegisterDefaultTranslations(validate, translator)

	addValidationRule("unique", models.UniqueColumnValidator, "{0} already exist")
	addValidationRule("exists", models.ExistColumnValidator, "{0} is invalid")

	initializeTemplates("app/views/")

	// Create a handler using the CORS middleware
	err := http.ListenAndServe(":80", router)
	if err != nil {
		log.Fatal(err)
	}
}

func initializeTemplates(viewPath string) {
	var err error
	// Parse the matched templates
	templates, err = template.ParseGlob(viewPath + "*.gohtml")
	if err != nil {
		log.Fatal(err)
	}
}

func (c *ControllerArg) View(filePath string, data interface{}) {
	// Execute the layout template, passing the about page template as the "content" section
	err := templates.ExecuteTemplate(c.Response, c.layout, struct {
		Body template.HTML
		Data interface{}
	}{
		Body: executeTemplateToString(templates, filePath+".gohtml", data),
		Data: data,
	})

	if err != nil {
		http.Error(c.Response, err.Error(), http.StatusInternalServerError)
	}
}

func (c *ControllerArg) ViewStatic(filePath string) {
	http.ServeFile(c.Response, c.Request, filePath)
}

func (c *ControllerArg) Validate(model interface{}) interface{} {
	err := json.NewDecoder(c.Request.Body).Decode(model)
	if err != nil {
		panic(fmt.Sprintf("Json decode: %s", err))
	}

	err = validate.Struct(model)
	if err != nil {
		errorBag := map[string]string{}
		validationErrors, ok := err.(validator.ValidationErrors)
		if !ok {
			// Return a generic error message if the error is not a validation error
			errorBag["general"] = "validation failed"
		}

		// Loop through each validation error and collect the field and error message
		for _, validationErr := range validationErrors {
			name := getFieldName(model, validationErr.Field())
			errorBag[name] = strings.Replace(validationErr.Translate(translator), validationErr.Field(), name, 1)
		}

		// Convert the errors to JSON format
		jsonData, err := json.Marshal(errorBag)
		if err != nil {
			panic(fmt.Sprintf("failed to marshal JSON: %v", err))
		}

		return string(jsonData)
	}

	return nil
}

func getFieldName(data interface{}, metaFieldName string) string {
	value := reflect.TypeOf(data)
	if value.Kind() == reflect.Ptr {
		value = value.Elem() // Dereference the pointer
	}

	if value.Kind() == reflect.Struct {
		for i := 0; i < value.NumField(); i++ {
			field := value.Field(i)
			if field.Name == metaFieldName {
				return field.Tag.Get("json")
			}
		}

		return ""
	}

	panic(fmt.Sprintf("Invalid type: expected struct '%s' given", value.Kind()))
}

func executeTemplateToString(t *template.Template, templateName string, data interface{}) template.HTML {
	var buf bytes.Buffer

	// Execute the template and write the output to the buffer
	err := t.ExecuteTemplate(&buf, templateName, data)
	if err != nil {
		return ""
	}

	// Return the buffer content as template.HTML, assuming it contains safe HTML
	return template.HTML(buf.String())
}

func (r *Router) initializeGroup() {
	r.namespace = "/"
	r.guards = map[string]GuardCallbackType{}
}

func logRequest(req *http.Request) {
	log.Printf("%s %s %s\n", req.RemoteAddr, req.Method, req.URL)
}

func getFunctionName(fn interface{}) string {
	return runtime.FuncForPC(reflect.ValueOf(fn).Pointer()).Name()
}

func addValidationRule(name string, fn validator.Func, message string) {
	_ = validate.RegisterValidation(name, fn)
	_ = validate.RegisterTranslation(name, translator, func(ut ut.Translator) error {
		return ut.Add(name, message, true) // see universal-translator for details
	}, func(ut ut.Translator, fe validator.FieldError) string {
		t, _ := ut.T(name, fe.Field())
		return t
	})
}
