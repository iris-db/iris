package view

var (
	DeleteWordSequence = NewEscapeSequence(Backspace)

	UpArrowSequence    = NewEscapeSequence(ArrowEscape, ArrowUp)
	DownArrowSequence  = NewEscapeSequence(ArrowEscape, ArrowDown)
	LeftArrowSequence  = NewEscapeSequence(ArrowEscape, ArrowLeft)
	RightArrowSequence = NewEscapeSequence(ArrowEscape, ArrowRight)

	Sequences = []*EscapeSequence{
		DeleteWordSequence,
		UpArrowSequence,
		DownArrowSequence,
		LeftArrowSequence,
		RightArrowSequence,
	}
)

// EscapeSequence represents a combination of characters pressed through an escape signal.
//
// Example: Command+Backspace = 27, 127
type EscapeSequence struct {
	sequence []rune // sequence is the escape characters.
	current  []rune // current is the actual sequence.
}

// NewEscapeSequence creates a new EscapeSequence of the specified length.
func NewEscapeSequence(sequence ...rune) *EscapeSequence {
	return &EscapeSequence{sequence: append([]rune{Escape}, sequence...)}
}

// Triggered returns true if the escape sequence was triggered.
func (e *EscapeSequence) Triggered() bool {
	if len(e.current) != len(e.sequence) {
		return false
	}

	for i := range e.sequence {
		if e.current[i] != e.sequence[i] {
			return false
		}
	}

	return true
}

func (e *EscapeSequence) Read(c rune) {
	if len(e.current) > len(e.sequence) {
		e.current = nil
	}
	e.current = append(e.current, c)
}
