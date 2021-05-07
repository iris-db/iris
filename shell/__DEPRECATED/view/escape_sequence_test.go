package view_test

import (
	. "github.com/iris-db/iris/shell-deprecated/view"
	"testing"
)

func TestDeleteWordSequence(t *testing.T) {
	DeleteWordSequence.Read(Escape)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("triggered before backspace sent")
	}

	DeleteWordSequence.Read(Backspace)
	if !DeleteWordSequence.Triggered() {
		t.Fatalf("not triggered after proper sequence")
	}

	DeleteWordSequence.Read(Escape)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("triggered after another escape sent")
	}

	DeleteWordSequence.Read(Backspace)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("did not trigger after 2nd sequence sent")
	}
}
