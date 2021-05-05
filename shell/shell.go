package main

// #include "key_reader.h"
import "C"
import (
	"fmt"
	"github.com/iris-db/iris/shell/term"
)

const (
	nullChar    = '\000'
	escape      = 27
	backspace   = 127
	whitespaceA = 32
	whitespaceB = 54
)

func main() {
	C.EnableRawMode()

	escaping := false
	isWriting := false

	words := term.NewWords()

	for {
		c := C.char(nullChar)

		C.ReadBytes((*C.char)(&c))

		if c == nullChar {
			continue
		}

		if C.CharEqual(c, whitespaceA) || C.CharEqual(c, whitespaceB) {
			words.NewWS()
		} else {
			if !isWriting {
				words.Increment()
			}
		}

		if escaping {
			if C.CharEqual(c, backspace) {
				isWriting = true
				for n := 0; n < words.PopLast(); n++ {
					fmt.Print("\b \b")
				}
				isWriting = false
				words.Reset()
			}
		}

		if C.CharEqual(c, backspace) {
			fmt.Print("\b \b")
			continue
		}

		fmt.Printf("%v", string(c))

		escaping = bool(C.CharEqual(c, escape))
		if escaping {
			continue
		}

		if C.CharEqual(c, 13) {
			break
		}
	}

	C.DisableRawMode()
}
