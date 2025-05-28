package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/signal"

	"github.com/cockroachdb/errors"
	openfga "github.com/openfga/go-sdk"
	"github.com/openfga/go-sdk/client"
	"github.com/openfga/language/pkg/go/transformer"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func serve(ctx context.Context) error {
	// Init OpenFGA
	fga, err := client.NewSdkClient(&client.ClientConfiguration{
        ApiUrl: "http://localhost:8080",  // NOTE: no trailing slash!
		AuthorizationModelId: "",  // defaults to the latest model version

        // StoreId: os.Getenv("FGA_STORE_ID"),
        // AuthorizationModelId: os.Getenv("FGA_MODEL_ID"),
		// Credentials: &credentials.Credentials{
        //     Method: credentials.CredentialsMethodApiToken,
        //     Config: &credentials.Config{
        //         ApiToken: os.Getenv("OPENFGA_API_TOKEN"), // will be passed as the "Authorization: Bearer ${ApiToken}" request header
        //     },
        // },
    })
	if err != nil {
		return errors.Wrap(err, "OpenFGA client init failed")
	}

	// Create a store
	// NOTE: don't create a store every time
	createStore, err := fga.CreateStore(ctx).Body(client.ClientCreateStoreRequest{Name: "FGA Demo"}).Execute()
    if err != nil {
		return errors.Wrap(err, "failed to create a store")
    }
	storeId := createStore.Id
	fmt.Printf("storeId: %+v\n", storeId)

	// Write the model
	var body client.ClientWriteAuthorizationModelRequest
	if err := json.Unmarshal([]byte(MODEL), &body); err != nil {
		return errors.Wrap(err, "failed to unmarshal model")
	}
	writeModel, err := fga.WriteAuthorizationModel(ctx).Options(client.ClientWriteAuthorizationModelOptions{
		StoreId: &storeId,
	}).Body(body).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to write auth model")
	}
	authModelId := writeModel.AuthorizationModelId
	fmt.Printf("authModelId: %+v\n", authModelId)

	// Read latest model
	authModel, err := fga.ReadLatestAuthorizationModel(ctx).Options(client.ClientReadLatestAuthorizationModelOptions{
		StoreId: &storeId,
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to get auth model")
	}
	fmt.Printf("authModel: %+v\n", authModel.AuthorizationModel.Id)//nocommit



	// Re-init client
	fga.SetStoreId(storeId)
	fga.SetAuthorizationModelId(authModelId)  // Defaults to the latest model version


	// Write tuples
	_, err = fga.Write(ctx).Body(client.ClientWriteRequest{
		Writes: []client.ClientTupleKey{
			// "user:1" can view the root
			{User: "user:1", Relation: "viewer", Object: "folder:/root"},
			// A folder has a subfolder (viewable)
			{User: "folder:/root#viewer", Relation: "viewer", Object: "folder:/root/project"},
			// Documents
			{User: "folder:/root", Relation: "parent_folder", Object: "document:README"},
			{User: "folder:/root/project", Relation: "parent_folder", Object: "document:main.go"},
		},
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to grant permission")
	}

	// Check
	checkResult, err := fga.Check(ctx).Options(client.ClientCheckOptions{
		// Cache will be used
		Consistency: openfga.CONSISTENCYPREFERENCE_MINIMIZE_LATENCY.Ptr(),
	}).Body(client.ClientCheckRequest{
		User: "user:1",
		Relation: "viewer",
		Object: "document:main.go",
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to check permissions")
	}
	fmt.Printf("Allowed: %+v\n", *checkResult.Allowed)

	// List objects
	listObjects, err := fga.ListObjects(ctx).Body(client.ClientListObjectsRequest{
		User: "user:1",
		Relation: "viewer",
		Type: "document",
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to list objects")
	}
	fmt.Printf("listObjects: %+v\n", listObjects.Objects)//nocommit

	// List users
	listUsers, err := fga.ListUsers(ctx).Body(client.ClientListUsersRequest{
		Object: *openfga.NewFgaObject("document", "main.go"),
		Relation: "viewer",
		UserFilters: []openfga.UserTypeFilter{
			// Return only specific subjects
			{ Type: "user" },
		},
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to list users")
	}
	fmt.Printf("listUsers: %+v\n", listUsers.Users)
	fmt.Printf("listUsers: %+v\n", listUsers.Users[0].Object)  // user:1

	// Write assertions
	fga.WriteAssertions(ctx).Body([]client.ClientAssertion{
		{User: "user:1", Relation: "viewer", Object: "document:README", Expectation: true},
	}).Execute()

	// Expand: returns all users that have a specific relationship with the object
	// To build an access graph, call expand on leaves recursively
	expandUsers, err := fga.Expand(ctx).Body(client.ClientExpandRequest{
		Relation: "viewer",
		Object: "document:README",
	}).Execute()
	if err != nil {
		return errors.Wrap(err, "failed to expand")
	}
	fmt.Printf("expandUsers: %+v\n", expandUsers.Tree.Root)//nocommit


	// Done
	return nil
}

func main() {
	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt, os.Kill)
	defer cancel()

	if err := serve(ctx); err != nil {
		log.Fatal().Err(err).Msg("App failed")
	}
}



var MODEL = transformer.MustTransformDSLToJSON(`
model
  schema 1.1

type user

type folder
  relations
    define viewer: [user, folder#viewer]

type document
  relations
    define parent_folder: [folder]
    define viewer: [user] or viewer from parent_folder

`)


func init(){
	// Nice colored logger
    log.Logger = zerolog.New(zerolog.NewConsoleWriter())
}


func p[T any](v T) *T {
	return &v
}