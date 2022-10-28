/*
Webserver serves a page on :1718 by default. It's an app to generate QR codes.
*/
package main

import (
	"flag"
	"html/template"
	"log"
	"net/http"
)

// Command-line flag
var bind = flag.String("addr", ":1718", "http service address") // Q=17, R=18

func main(){
	// Parse command-line flags
	flag.Parse()

	// Serve page
	http.Handle("/", http.HandlerFunc(indexPage))
	err := http.ListenAndServe(*bind, nil)
	if err != nil {
		log.Fatal("ListenAndServe:", err)
	}
}

func indexPage(w http.ResponseWriter, req *http.Request){
	// "s": GET parameter, it sent by the form
	// Render the template, using the form value as the "current" (".") data item
	indexPageTemplate.Execute(w, req.FormValue("s"))
}

// HTML template.
// `Must()` panics if the result is non-nil
// New(name) is the template name
var indexPageTemplate = template.Must(template.New("qr").Parse(indexPageTemplateStr))

// index page
const indexPageTemplateStr = `
<!DOCTYPE html>
<html>
<head>
	<title>QR Link Generator</title>
</head>
<body>
	{{if .}}
		<img src="http://chart.apis.google.com/chart?chs=300x300&cht=qr&choe=UTF-8&chl={{.}}" />
		<br>
		{{.}}
		<br>
		<br>
	{{end}}

	<form action="/" name=f method="GET">
		<input maxLength=1024 size=70 name="s" value="" title="Text to QR Encode">
		<input type=submit value="Show QR" name="qr">
	</form>
</body>
</html>
`