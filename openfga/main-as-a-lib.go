package main

import (
	"context"
	"fmt"

	openfgav1 "github.com/openfga/api/proto/openfga/v1"
	"github.com/openfga/language/pkg/go/transformer"
	"github.com/openfga/openfga/pkg/server"
	"github.com/openfga/openfga/pkg/storage/memory"
)

func main(){
	ctx := context.Background()

	// Init datastore
	datastore := memory.New() // other supported datastores include Postgres, MySQL and SQLite
	defer datastore.Close()

	openfga, err := server.NewServerWithOpts(
		server.WithDatastore(datastore),
		server.WithCheckQueryCacheEnabled(true),
		// more options available
	)
	if err != nil {
		panic(err)
	}
	defer openfga.Close()

	// create store
	store, err := openfga.CreateStore(ctx,
		&openfgav1.CreateStoreRequest{Name: "demo"},
	)
	if err != nil {
		panic(err)
	}

	model := transformer.MustTransformDSLToProto(`
		model
		  schema 1.1

		type user

		type document
		  relations
		    define reader: [user]
	`)

	// write the model to the store
	authModel, err := openfga.WriteAuthorizationModel(ctx, &openfgav1.WriteAuthorizationModelRequest{
		StoreId:         store.GetId(),
		TypeDefinitions: model.GetTypeDefinitions(),
		Conditions:      model.GetConditions(),
		SchemaVersion:   model.GetSchemaVersion(),
	})
	if err != nil {
		panic(err)
	}
	fmt.Printf("authModel: %+v\n", authModel)//nocommit

	// write tuples to the store
	_, err = openfga.Write(context.Background(), &openfgav1.WriteRequest{
		StoreId: store.GetId(),
		Writes: &openfgav1.WriteRequestWrites{
			TupleKeys: []*openfgav1.TupleKey{
				{Object: "document:budget", Relation: "reader", User: "user:anne"},
			},
		},
		Deletes: nil,
	})
	if err != nil {
		panic(err)
	}

	// Check
	checkResp, err := openfga.Check(ctx, &openfgav1.CheckRequest{
		StoreId: store.Id,
		AuthorizationModelId: authModel.AuthorizationModelId,
		Consistency: openfgav1.ConsistencyPreference_MINIMIZE_LATENCY,
		TupleKey: &openfgav1.CheckRequestTupleKey{
			User: "user:anne",
			Relation: "reader",
			Object: "document:budget",
		},
	})
	if err != nil {
		panic(err)
	}
	fmt.Printf("checkResp: %+v\n", checkResp)
}