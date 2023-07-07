package home

import (
	"github.com/ghostff/payment-frame/app/utils/route"
)

func Index(r *route.ControllerArg) {
	r.ViewStatic("public/")
}

/*func (m controllers.Controller) Browse(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Method, "Browse")
}*/

/*func (m *Controller) Read(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Method, "Read")
}

func (m *Controller) Edit(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Method, "Edit")
}

func (m *Controller) Add(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Method, "Add")
}

func (m *Controller) delete(w http.ResponseWriter, r *http.Request) {
	fmt.Println(r.Method, "delete")
}*/
