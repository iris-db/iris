package view

// Arrow keys.
const (
	ArrowEscape = 91

	ArrowLeft  = 68
	ArrowRight = 67
	ArrowDown  = 66
	ArrowUp    = 65
)

// Common keys.
const (
	Null      = '\000'
	Escape    = 27
	Backspace = 127
)

// Cursor directions.
const (
	CursorRight = "\033[C"
	CursorLeft  = "\033[D"
	CursorUp    = "\033[A"
	CursorDown  = "\033[;H"
)

func IsEscapeSequence(c rune) bool {
	switch c {
	case Escape, ArrowEscape, Backspace:
		return true
	default:
		return false
	}
}
