package main

import (
	"testing"
)

func TestUnmarshallNonQuotedBSON(t *testing.T) {
	data := `{ hello: "world" }`

	ptr := UnmarshallNonQuotedBSON(data)
	if ptr == nil {
		t.Fatal("received a null pointer")
	}

	res := stringFromCharPtr(ptr)

	expected := `{"hello":"world"}`

	if res != expected {
		t.Fatalf("got %s; expected %s", res, expected)
	}
}
