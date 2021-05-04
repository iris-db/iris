package main

import (
	"C"
	"go.mongodb.org/mongo-driver/bson"
	nonstandard "gopkg.in/mgo.v2/bson"
)

//export UnmarshallNonQuotedBSON
// UnmarshallNonQuotedBSON converts a non standard JSON string, such as { key: "value" }, to a valid JSON string.
func UnmarshallNonQuotedBSON(v string) *C.char {
	var data interface{}

	err := nonstandard.UnmarshalJSON([]byte(v), &data)
	if err != nil {
		return nil
	}

	json, err := bson.MarshalExtJSON(data, true, true)
	if err != nil {
		return nil
	}

	return C.CString(string(json))
}

// stringFromCharPtr converts a *C.char to a string.
func stringFromCharPtr(ptr *C.char) string {
	return C.GoString(ptr)
}

func main() {}
