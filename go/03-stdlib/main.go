package main

// Run me:
// $ go run .

import (
	"log"
)

func main(){
	log.SetFlags(log.Lshortfile | log.Lmsgprefix)

	PlayFmt()
	PlayBuiltin()
	PlayBytes()
	PlayEmbed()
	PlayEncodingJson()
	PlayHtml()
	PlayHtmlTemplate()
	PlayOS()
	PlayOsExec()
	PlayIO()
	PlayIOFS()
	PlayReflect()
	PlayRegexp()
	
}
