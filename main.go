package main

import (
	"fmt"
	"github.com/ghostff/payment-frame/app/models"
	"github.com/ghostff/payment-frame/app/routes"
	"github.com/ghostff/payment-frame/app/utils/route"
	"github.com/joho/godotenv"
	"os"
)

func main() {
	// load .env file
	err := godotenv.Load(".env")
	if err != nil {
		fmt.Printf("Could not read env: %s", err.Error())
		os.Exit(1)
	}
	models.Boostrap()
	route.Boostrap(routes.RegisterRoutes)
}
