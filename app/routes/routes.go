package routes

import (
	"github.com/ghostff/payment-frame/app/controllers/api/auth"
	webHome "github.com/ghostff/payment-frame/app/controllers/web/home"
	"github.com/ghostff/payment-frame/app/utils/route"
	"github.com/rs/cors"
	"net/http"
)

func mustBeJson(s route.ControlScope, w http.ResponseWriter, r *http.Request) bool {
	return false
}

func RegisterRoutes(router *route.Router) {
	router.Get("/", webHome.Index)

	//router.Get("so/{what}", webHome.Index)

	router.Group("api", nil, func(router *route.Router) {
		router.Cors(cors.AllowAll()).Post("register", auth.Register)
		router.Cors(cors.AllowAll()).Post("login", auth.Login)
	})

	/*router.Group("api", route.GuardType{route.IsJSONRequest}, func(router *route.Router) {
		router.Cors(cors.AllowAll()).Post("register", auth.Register)
		router.Cors(cors.AllowAll()).Post("login", auth.Login)
		router.Get("account/{name}/with/{id:\\d+}", apiHome.Index)
		//router.Get("hello/{email}", apiHome.Index)
		//router.Get("my/name/is/{token}?", apiHome.Index)

		//
		router.Group("foo", nil, func(router *route.Router) {
			//router.Get("account", apiHome.Index)
		})
	})

	router.Get("account/{name}/with/{id:\\d+}", apiHome.Index)
	//router.Get("{all:.*}", webHome.Index)*/
}
