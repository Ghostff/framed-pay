package home

import (
	"fmt"
	"github.com/ghostff/payment-frame/app/models"
	"github.com/ghostff/payment-frame/app/utils/route"
)

func Index(arg *route.ControllerArg) {
	var user models.User
	if !arg.ValidatePost(&user) {
		return
	}

	fmt.Println(user)

	/*if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, "Failed to decode request body: %v", err)
		return
	}

	// Validate the user input
	if err := validateUser(user); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, "Invalid user input: %v", err)
		return
	}

	// Process the valid user input
	// ...

	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "User created successfully")

	arg.Request.ParseForm()*/
	fmt.Println(arg.Request.Form)
	fmt.Fprintf(arg.Response, "{\"name\":\"John\"}")
}
