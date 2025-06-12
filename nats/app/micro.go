package main

import (
	"context"
	"encoding/json"
	"fmt"
	"slices"
	"time"

	"github.com/cockroachdb/errors"
	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/micro"
)

func microserviceServe(ctx context.Context, k *nats.Conn) error {
	srv, err := micro.AddService(k, micro.Config{
		Name:        "minmax",
		Version:     "0.0.1",
		Description: "Returns the min/max number in a request",
		// Will by default listen on topic <group>.<endpoint>
	})
	if err != nil {
		return errors.Wrap(err, "AddService() failed")
	}

	// Register microservice APIs
	root := srv.AddGroup("minmax")
	root.AddEndpoint("min", micro.HandlerFunc(handleMin))
	root.AddEndpoint("max", micro.HandlerFunc(handleMax))


	// Now make a request
	requestData, _ := json.Marshal([]int{-1, 2, 100, -2000})
	msg, _ := k.Request("minmax.min", requestData, 2*time.Second)
	var res ServiceResult
	json.Unmarshal(msg.Data, &res)
	fmt.Printf("microservice response: %+v\n", res)//nocommit


	// Done
	return nil
}



func handleMin(req micro.Request) {
	// JSON input
	var arr []int
	_ = json.Unmarshal([]byte(req.Data()), &arr)
	slices.Sort(arr)

	// Result
	res := ServiceResult{Min: arr[0]}
	req.RespondJSON(res)
}


func handleMax(req micro.Request) {
	// JSON input
	var arr []int
	_ = json.Unmarshal([]byte(req.Data()), &arr)
	slices.Sort(arr)


	// Result
	res := ServiceResult{Max: arr[len(arr)-1]}
	req.RespondJSON(res)
}

type ServiceResult struct {
	Min int `json:"min,omitempty"`
	Max int `json:"max,omitempty"`
}