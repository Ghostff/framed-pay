package models

type User struct {
	FirstName          string `json:"first_name" validate:"required,min=2,max=255"`
	LastName           string `json:"last_name" validate:"required,min=2,max=255"`
	Email              string `json:"email" validate:"required,email,unique=users:email"`
	Password           string `json:"password" validate:"required,min=8,max=200"`
	PasswordResetToken string
	Model
}
