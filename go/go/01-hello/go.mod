module example.com/hello

go 1.18

// $ go mod init example.com/hello
// $ vim $hello.go

// $ cd greetings 
// $ go mod init example.com/greetings
// $ vim greetings.go
// $ cd ..

// $ go mod tidy
// $ go build
// $ ./hello

// Install?
// Install target:
// $ go list -f '{{.Target}}'
// $ go install

replace example.com/greetings => ./greetings

require rsc.io/quote v1.5.2

require example.com/greetings v0.0.0-00010101000000-000000000000

require (
	golang.org/x/text v0.0.0-20170915032832-14c0d48ead0c // indirect
	rsc.io/sampler v1.3.0 // indirect
)
