package raw

import "testing"

func TestDeleteWordSequence(t *testing.T) {
	DeleteWordSequence.Read(escape)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("triggered before backspace sent")
	}

	DeleteWordSequence.Read(backspace)
	if !DeleteWordSequence.Triggered() {
		t.Fatalf("not triggered after proper sequence")
	}

	DeleteWordSequence.Read(escape)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("triggered after another escape sent")
	}

	DeleteWordSequence.Read(backspace)
	if DeleteWordSequence.Triggered() {
		t.Fatalf("did not trigger after 2nd sequence sent")
	}
}
