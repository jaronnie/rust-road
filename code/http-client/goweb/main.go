package main

import (
	"fmt"

	"github.com/gin-gonic/gin"
)

type Data struct {
	Name string `json:"name"`
}

type Response struct {
	Message string `json:"message"`
}

func main() {
	g := gin.Default()

	g.GET("/json", func(ctx *gin.Context) {
		greet := Data{
			Name: "jaronnie",
		}
		ctx.IndentedJSON(200, greet)
	})

	g.POST("/post", func(ctx *gin.Context) {
		var data Data
		ctx.Bind(&data)

		fmt.Printf("%#v\n", data)
		ctx.IndentedJSON(200, Response{Message: "ok"})
	})

	g.Run(":9090")
}
