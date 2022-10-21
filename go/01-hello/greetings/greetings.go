package greetings

// Import a package -- from standard library
import (
	"errors"
	"fmt"
	"math/rand"
)

// A function returns a string and error struct
func HailByName(name string) (string, error) {
	// Error handling
	if name == "" {
		return "", errors.New("no name provided")
	}

	// var message string
	message := fmt.Sprintf(randomGreetMessage(), name)
	return message, nil
}


// Greet multiple people
// Returns: map { name => greeting }
func HailByNames(names []string) (map[string]string, error){
	// Results: mapping { name => greeting }
	messages := make(map[string]string)  // map[key-type] value-type

	for _, name := range names {
		message, err := HailByName(name)
		if err != nil {
			return nil, err
		}

		messages[name] = message
	}

	return messages, nil
}

// local function: starts with a lowercase letter
func randomGreetMessage() string {
	// Declare a slice.
	// Empty [] means its size can be changed.
	messages := []string{
		"Hi %v!",
		"Hello %v!",
		"Greetings %v!",
	}
	return messages[rand.Intn(len(messages))]
}