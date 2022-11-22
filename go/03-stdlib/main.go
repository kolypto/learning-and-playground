package main

// Run me:
// $ go run .
// $ go test

// Build or test with race detector:
// $ go test -race mypkg
// $ go run -race mysrc.go
// $ go build -race mycmd
// $ go install -race mypkg

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
	PlayTrace()
	PlaySort()
	PlayCustomSort()
	PlaySync()
	PlayTime()

	PlayNet()
	PlayHttp()

	PlayRPC()
}
