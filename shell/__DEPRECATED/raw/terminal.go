package raw

// #include "raw_terminal_input.h"
import "C"
import (
	"fmt"
	"github.com/iris-db/iris/shell-deprecated/view"
)

// Terminal is a raw terminal implementation.
type Terminal struct {
	writing  bool
	escaping bool
	cursor   *view.Cursor
}

const (
	null      = '\000'
	escape    = 27
	backspace = 127
)

// NewTerminal creates a new terminal.
func NewTerminal() *Terminal {
	return &Terminal{
		writing:  false,
		escaping: false,
		cursor:   view.NewCursor(),
	}
}

// Start starts the terminal.
func (t *Terminal) Start() {
	fmt.Println("Iris shell version v.4.2.0")
	fmt.Println("connecting to iris://shard-cluster-00-00.xc523wc.iris")
	fmt.Println("CONNPOOL Connected successfully to shard C-00-00")
	fmt.Print("Iris Enterprise C-00-00:PRIMARY> ")

	cursor, _, escaping := t.cursor, t.writing, t.escaping

	C.EnableRawMode()

	for {
		rc := C.char(null)

		C.ReadBytes((*C.char)(&rc))

		if rc == null {
			continue
		}

		c := rune(int(rc))

		cursor.PushChar(c)

		if view.LeftArrowSequence.Triggered() {
			cursor.DecrementPos()
			fmt.Print("\000\000")
			continue
		}

		if view.RightArrowSequence.Triggered() {
			cursor.IncrementPos()
			continue
		}

		if C.CharEqual(rc, backspace) {
			l := 1
			if escaping {
				l = cursor.GetWordDeleteLength()
			}

			for n := 0; n < l; n++ {
				t.delete()
			}
			continue
		}

		if C.CharEqual(rc, '\r') {
			t.write("\r\n")
			t.write("Iris Enterprise C-00-00:PRIMARY> ")
			t.cursor.Reset()
			continue
		}

		t.write(string(c))

		escaping = bool(C.CharEqual(rc, escape))
		if escaping {
			continue
		}

		if C.CharEqual(rc, 'q') {
			fmt.Println()
			break
		}
	}

	C.DisableRawMode()
}

func (t *Terminal) write(v ...interface{}) {
	t.writing = true
	fmt.Print(v...)
	t.writing = false
}

// delete deletes a single char from the STDIN.
func (t *Terminal) delete() {
	if t.cursor.Pos() > 0 {
		t.write("\b \b")
		t.cursor.RemoveChar()
	}
}
