package main

import (
	"fmt"
	"io"
	"io/fs"
	"log"
	"os"
	"os/exec"
	"strings"
)

func PlayOS(){
	// Open a file and read it
	file, err := os.Open("os.go")
	if err != nil {
		log.Fatal(err)
	}

	buf := make([]byte, 100)
	count, err := file.Read(buf)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Read %d bytes: %q\n", count, buf)

	// Read file, quick
	contents, err := os.ReadFile("os.go")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Read file: %q\n", contents[:20])

	// Write file, quick
	err = os.WriteFile("/tmp/example.txt", []byte("example"), 0644)
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove("/tmp/example.txt")

	// Write to a temporary file
	f, err := os.CreateTemp("", "example.*.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove(f.Name())

	if _, err := f.Write([]byte("content")); err != nil {
		f.Close()
		log.Fatal(err)
	}

	// User cache directory
	cachedir, err := os.UserCacheDir()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Cache dir: %s\n", cachedir)

	// Command-line args
	fmt.Println(os.Args)
	executable, err := os.Executable()
	fmt.Printf("Executable: argv[0]=%s\n", executable)


	// Environment
	fmt.Printf("SHELL=%s\n", os.Getenv("SHELL"))
	fmt.Println(os.ExpandEnv("SHELL=$SHELL"))
	value, ok := os.LookupEnv("USER")
	if ok {
		fmt.Printf("$USER=%s\n", value)
	}

	// List dir
	files, err := os.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}
	for _, file := range files {
		fmt.Printf("File: %s\n", file.Name())
	}
}


func PlayOsExec(){
	// Execute command with arguments
	cmd := exec.Command("hostname")
	if cmd.Err != nil {
		log.Fatal(cmd.Err)
	}
	output, err := cmd.CombinedOutput()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Exec: %s\n", output)
}


func PlayIO(){
	// Copy(): copy from one stream to another
	r := strings.NewReader("data to be read\n")
	if _, err := io.Copy(os.Stdout, r); err != nil {
		log.Fatal(err)
	}
}

func PlayIOFS(){
	rootFs := os.DirFS("/")

	fs.WalkDir(rootFs, "mnt", func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Walk file: %s\n", path)
		return nil
	})
}