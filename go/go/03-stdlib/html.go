package main

import (
	"fmt"
	"html"
	"html/template"
	"log"
	"os"
)


func PlayHtml(){
	// HTML escape
	escaped := html.EscapeString("<tag>")
	fmt.Printf("escaped=%q\n", escaped)  //-> "&lt;tag&gt;"

	// HTML unescape
	unescaped := html.UnescapeString(escaped)
	fmt.Printf("unescaped=%q\n", unescaped)  //-> "<tag>"
}


func PlayHtmlTemplate(){
	// "html/template" provides the same interface as "text/template", but for HTML.
	// Plain text is escaped. Escaping is contextual: HTML, JavaScript, CSS, URI contexts (uses proper sanitizing)
	// Assumption: the template is trusted, the parameters are not.

	template.HTMLEscape(os.Stdout, []byte("<a>")) //-> &lt;a&gt;
	template.JSEscape(os.Stdout, []byte("<a>")) //-> \u003Ca\u003E
	fmt.Println(template.URLQueryEscaper("<a>")) //-> %3Ca%3E
	
	// I.e. This is what happens:
	//   <a href="/search?q={{.}}">{{.}}</a> 
	//   <a href="/search?q={{. | urlescaper | attrescaper}}">{{. | htmlescaper}}</a>
	// "href" enters the URI namespace. So do "data-href" and "my:href"

	{
		// Template.
		// {{...}}} is the action that gets evaluated
		// {{.}} prints the cursor
		// {{23 -}} < {{- 45}}  -- trims whitespace after, and before, the action
		// {{$variableName.fieldName}} and {{$variableName.$keyName}}
		// {{$object.Method "arg"}}  
		// -- the method mey return `_, error` 
		// -- in case of an error: execution terminates, and an error is returned by Execute()
		//
		// {{/* a comment */}}
		// {{pipeline}}  -- will use fmt.Print() 
		//
		// {{if pipeline}} T1 {{else}} T2 {{end}}
		// {{if pipeline}} T1 {{else if pipeline}} T2 {{end}}
		//
		// {{range pipeline}} item {{else}} empty {{end}}  --  iterate, set dot to the current value
		// {{break}}, {{continue}}  -- control structures for range
		//
		// {{with pipeline}} T1 {{else}} no value {{end}}  -- set dot to the value of the pipeline. If empty, do "else" (if present)
		// {{template "name"}} -- execute template with nil data
		// {{template "name" pipeline}}  -- execute template with dot set to the value of the `pipeline`
		// {{block "name" pipeline}} T1 {{end}}   -- a shorthand for defining a template and then executing it in place:
		// {{define "name"}} T1 {{end}}{{template "name" pipeline}}
		//
		// Pipeline: {{ argument.Method "arg" | ...}}
		// In a chained pipeline, the result of each command is passed as the last argument of the following command
		//
		// Variables:
		// {{range $index, $element := pipeline}}
		// {{with $x := "output" | printf "%q"}}{{$x}}{{end}}
		//
		// Strings and printing:
		// {{"\"output\""}}
		// {{`"output"`}}
		// {{printf "%q" "output"}}
		// {{"output" | printf "%q"}}
		// {{printf "%q" (print "out" "put")}}
		const templateString = `
		{{define "Named-Block"}}
			Hello, {{.}}!
		{{end}}
		`

		// Create a named template
		// Template names form a namespace of templates. You can evaluate them by name.
		// Must() panics if the error is non-nil.
		t := template.Must(template.New("name").Parse(templateString))

		// Fail when a key is missing.
		t.Option("missingkey=error")  

		// Execute a template. Use data object "T"
		err := t.ExecuteTemplate(os.Stdout, "Named-Block", "John")  //-> Hello, John!
		if err != nil {
			log.Fatal(err)
		}
		
		// Insert literal HTML (known safe HTML)
		// See also: CSS(), HTMLAttr(), JS(), JSStr(), Srcset(), URL()  -- known safe elements
		err = t.ExecuteTemplate(os.Stdout, "Named-Block", template.HTML(`<b>John</b>`))  //-> Hello, <b>John</b>!
		if err != nil {
			log.Fatal(err)
		}
	}


	// Template features
	{
		const templateString = `
			<html>
			<head>
				<title>{{.Title}}</title>
			</head>
			<body>

			{{range .Items}}
				<div>{{.}}</div>
			{{else}}
				<p>No items
			{{end}}
		`

		// Prepare the template
		t, err := template.New("page").Parse(templateString)
		if err != nil {
			log.Fatal(err)
		}

		// Prepare the data
		data := struct {
			Title string
			Items []string
		}{
			Title: "My page",
			Items: []string{
				"My Photos",
				"My Blog",
			},
		}

		// Render
		err = t.Execute(os.Stdout, data)
		if err != nil {
			log.Fatal(err)
		}
	}
}
