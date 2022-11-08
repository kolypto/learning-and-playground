package main

import (
	"fmt"
	"reflect"
)

func PlayReflect(){
	// Inspect value type.
	var v any = "hey"
	value := reflect.ValueOf(v)  // get `Value`

	if value.Kind() == reflect.String {
		fmt.Printf("Is string: %s\n", value.String())
	}

	// Deep equal
	var a = []int{1, 2, 3}
	var b = []int{1, 2, 3}
	fmt.Printf("Deeply equal: %t", reflect.DeepEqual(a, b))

	// MapIter: iterator for ranging over a map
	var m = map[string]int{"a": 1, "b": 2, "c": 3}
	iter := reflect.ValueOf(m).MapRange()
	for iter.Next() {
		k := iter.Key()
		v := iter.Value()
		fmt.Printf("Key [%q]=%d\n", k.String(), v.Int())	
	}

	// Inspect a struct
	v = struct{
		Name string
		Age int64
	}{"John", 32}
	typ := reflect.ValueOf(v).Type()
	for i:=0; i<typ.NumField(); i++ {
		field := typ.Field(i)
		fmt.Printf("Field %s: %s\n", field.Name, field.Type.Kind())
	}
}