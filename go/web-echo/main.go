// Install: $ go get github.com/labstack/echo/v4

package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	// HelloWorldWebServer()
	// BindingServer()
	// ContextServer()
	ErrorHandlingServer()
}

const BIND_TO = ":1323"

// Topic: introduction
func HelloWorldWebServer() {
	e := echo.New()

	// Hello world
	e.GET("/", func(c echo.Context) error {
		return c.HTML(http.StatusOK, `
			<h1>Hello</h1>
			<a href="/users/1">User #1</a>
		`)
	})

	// Input: path param, query, form, file
	route := e.GET("/users/:id", func(c echo.Context) error {
		// Path parameter
		id := c.Param("id")

		// ?debug
		debug := c.QueryParam("debug")
		c.Logger().Debugf("Debug enabled: %v", debug)

		// Form value
		_ = c.FormValue("name")
		_, err := c.FormFile("avatar") // file
		if err != nil {
			c.Logger().Errorf("File upload failed: %v", err) // http.ProtocolError
			// return err
		}

		// Return JSON
		type User struct {
			Id    string `json:"id"`
			Name  string `json:"name" form:"name" query:"name"`
			Email string `json:"email" form:"email" query:"email"`
		}
		user := User{Id: id, Name: "John", Email: "john@example.com"}
		return c.JSON(http.StatusOK, user)
	})
	route.Name = "get-user-by-id" // may be useful for URL generation

	// Serve static files
	e.Static("/static", "static")
	e.File("/index.html", "public/index.html")

	// Middleware
	// You can also use Group-level middleware (on "/admin") or route-level middleware (per endpoint)
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	// Don't show the start banner
	e.HideBanner = true

	// Enable debug: changes log level
	e.Debug = true

	// Change logging
	e.Logger.SetHeader("${time_rfc3339} ${level}")
	e.Logger.SetOutput(os.Stdout)

	// Start the server
	e.Logger.Fatal(e.Start(BIND_TO))

	// A better way
	if err := e.Start(BIND_TO); err != http.ErrServerClosed {
		log.Fatal(err)
	}

	// Using http.Server
	s := http.Server{
		Addr:    BIND_TO,
		Handler: e,
	}
	if err := s.ListenAndServe(); err != http.ErrServerClosed {
		log.Fatal(err)
	}

	// NOTE: it can also to TLS and automatic certificate installation from Let'sEncrypt ;)
}

// Topic: binding
func BindingServer() {
	e := echo.New()
	e.Debug = true

	// Binding: bind path/query/header/request values to a struct
	// NOTE on security: don't pass input structs around. Have a separate struct for binding and map it explicitly to your business struct.

	e.GET("/", func(c echo.Context) error {
		// Tell the binder to bind the query string parameter `id` to its string field
		type User struct {
			// Data sources: query, param, header, json, xml, form
			// Note that binding at each stage will overwrite data bound in a previous stage
			ID string `path: "id" query:"id"`
		}

		// Bind
		var user User
		if err := c.Bind(&user); err != nil {
			c.Error(err)
			return nil
		}

		// It is also possible to bind directly: headers are not bound by default
		err := (&echo.DefaultBinder{}).BindHeaders(c, &user)
		if err != nil {
			return err
		}

		// Fluent manual binder
		var userId int64
		err = echo.QueryParamsBinder(c).
			Int64("id", &userId).
			FailFast(true). // stop on first error
			BindError()     // returns the first binding error
		if err != nil {
			return err
		}
		c.Logger().Infof("Bound user id: %v", userId)

		// Respond
		err = c.JSON(http.StatusOK, user)
		if err != nil {
			c.Logger().Errorf("Failed: %v", err)
		}
		return nil
	})

	e.Logger.Fatal(e.Start(BIND_TO))

}

// To bind a custom parameter: implement the interface
type Timestamp time.Time

func (t *Timestamp) UnmarshalParam(src string) error {
	ts, err := time.Parse(time.RFC3339, src)
	*t = Timestamp(ts)
	return err
}

// Topic: Context
func ContextServer() {
	e := echo.New()
	// e.Debug = true

	// Context: the context of the current HTTP request
	// It is an interface: easy to extend it with custom APIs

	// Define a custom context
	type CustomContext struct {
		echo.Context
	}

	// Now use a middleware to wrap a context: next(&CustomContext{c})
	e.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			cc := &CustomContext{c}
			return next(cc)
		}
	})

	// Use the custom context
	e.GET("/", func(c echo.Context) error {
		cc := c.(*CustomContext)
		return c.String(http.StatusOK, fmt.Sprintf("Context type: %T", cc))
	})

	e.Logger.Fatal(e.Start(BIND_TO))
}

// Topic: error handling
func ErrorHandlingServer() {
	e := echo.New()
	e.Debug = true

	// You can return `error` (results in 500) or echo.NewHTTPError()
	e.GET("/", func(c echo.Context) error {
		// Return `error`: results in status=500
		// { "error": "failed", "message": "Internal Server Error" }
		// return errors.New("failed")

		// Return a custom error code
		// { "error": "code=401, message=Please sign in", "message": "Please sign in" }
		// The "error" is only reported in Debug mode
		return echo.NewHTTPError(http.StatusUnauthorized, "Please sign in")
	})

	// Set a custom http error handler
	e.HTTPErrorHandler = func(err error, c echo.Context) {
		c.Logger().Error(err)

		// Send custom JSON
		c.JSON(http.StatusInternalServerError, map[string]any{
			"message": err.Error(),
			"code":    31337,
		})
	}

	e.Logger.Fatal(e.Start(BIND_TO))
}

// READ MORE: Middleware: https://echo.labstack.com/middleware/
// READ MORE: Cookbook: https://echo.labstack.com/cookbook/
