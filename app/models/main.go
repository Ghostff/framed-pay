package models

import (
	"fmt"
	"github.com/go-playground/validator/v10"
	"github.com/google/uuid"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"os"
	"strings"
	"time"
)

var db *gorm.DB

func AutoMigrate(model interface{}) {
	db.Migrator().AutoMigrate(model)
}

type Model struct {
	ID        uuid.UUID `gorm:"type:uuid;primaryKey;default:gen_random_uuid()"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt `gorm:"index"`
}

func Boostrap() {
	dsn := fmt.Sprintf(
		"host=%s user=%s password=%s dbname=%s port=%s sslmode=disable TimeZone=%s",
		os.Getenv("DB_HOST"),
		os.Getenv("DB_USERNAME"),
		os.Getenv("DB_PASSWORD"),
		os.Getenv("DB_DATABASE"),
		os.Getenv("DB_PORT"),
		os.Getenv("APP_TIMEZONE"),
	)

	var err error
	db, err = gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		fmt.Printf("db connection failed: %s", err.Error())
		os.Exit(1)
	}

	println("connected")
}

func (m *Model) Find(query interface{}, args ...interface{}) (tx *gorm.DB) {
	return db.Where(query, args...).Find(m)
}

func (m *Model) Exist(query interface{}, args ...interface{}) bool {
	return db.Where(query, args...).Find(m).RowsAffected > 0
}

func (m *Model) Create(data interface{}) (tx *gorm.DB) {
	return db.Create(data)
}

func UniqueColumnValidator(fl validator.FieldLevel) bool {
	options := strings.Split(fl.Param(), ":")
	if len(options) < 1 {
		panic(fmt.Sprintf("no table or column specified for exist. expected exists=table:colum, %s given", fl.Param()))
	}

	value := fl.Field().String()
	var result int64
	db.Table(options[0]).Select("COUNT(*)").Where(options[1]+" = ?", value).Count(&result)

	return result == 0
}

func ExistColumnValidator(fl validator.FieldLevel) bool {
	return !UniqueColumnValidator(fl)
}
