package main

import (
	"embed"
	"fmt"
)

// Here's how you embed a file into a variable
//go:embed main.go
var maingo string

// Embed into a filesystem
// Use `path.Match` patterns. 
// Directories are included recursively (excluding ".*" and "_*")
//go:embed *.go
var f embed.FS

func PlayEmbed(){
	// Read string
	fmt.Println("embed", maingo[:12])
	
	// Read file
	data, _ := f.ReadFile("main.go")
	println("embed", string(data[:12]))
}