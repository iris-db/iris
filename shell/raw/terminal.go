package raw

// #include "key_reader.h"
import "C"
import (
	"fmt"
)

// Terminal is a raw terminal implementation.
type Terminal struct {
	writing  bool
	escaping bool
	cursor   *cursor
}

const (
	NULL      = '\000'
	ESCAPE    = 27
	BACKSPACE = 127
)

// NewTerminal creates a new terminal.
func NewTerminal() *Terminal {
	return &Terminal{
		writing:  false,
		escaping: false,
		cursor:   newCursor(),
	}
}

// Start starts the terminal.
func (t *Terminal) Start() {
	C.EnableRawMode()

	cursor, writing, escaping := t.cursor, t.writing, t.escaping

	for {
		rc := C.char(NULL)

		C.ReadBytes((*C.char)(&rc))

		if rc == NULL {
			continue
		}

		c := string(rune(int(rc)))

		if !writing {
			cursor.PushChar(c)
		}

		if C.CharEqual(rc, BACKSPACE) {
			l := 1
			if escaping {
				l = cursor.GetWordDeleteLength()
			}

			for n := 0; n < l; n++ {
				t.delete()
			}
			continue
		}

		t.write(c)

		escaping = bool(C.CharEqual(rc, ESCAPE))
		if escaping {
			continue
		}

		if C.CharEqual(rc, 13) {
			break
		}
	}

	C.DisableRawMode()
}

func (t *Terminal) write(c string) {
	t.writing = true
	fmt.Print(c)
	t.writing = false
}

// delete deletes a single char from the STDIN.
func (t *Terminal) delete() {
	t.write("\b \b")
	t.cursor.RemoveChar()
}
