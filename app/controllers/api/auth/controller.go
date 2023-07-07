package auth

import (
	"fmt"
	"github.com/ghostff/payment-frame/app/models"
	"github.com/ghostff/payment-frame/app/utils/route"
	"golang.org/x/crypto/bcrypt"
)

func Login(arg *route.ControllerArg) {

}

func Register(arg *route.ControllerArg) {
	user := &models.User{}
	if err := arg.Validate(user); err != nil {
		fmt.Fprintf(arg.Response, `{"errors": %s}`, err)
	}

	bytes, _ := bcrypt.GenerateFromPassword([]byte(user.Password), 14)
	user.Password = string(bytes)

	result := user.Create(user)

	fmt.Println(user, result)
}
