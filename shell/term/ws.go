package term

// Words is a type to keep track of the current words on the current line of STDOUT. This is useful for deleting a full
// word when the proper escape sequence is sent.
type Words struct {
	pos []int // pos are the word positions.
}

func NewWords() *Words {
	return &Words{
		pos: []int{0},
	}
}

// NewWS tracks a new whitespace.
func (w *Words) NewWS() {
	w.pos = append(w.pos, 0)
}

// Increment increments the length of the current word.
func (w Words) Increment() {
	w.pos[w.current()] += 1
}

func (w Words) PopLast() int {
	return w.pos[w.current()] - 1
}

func (w Words) Reset() {
	w.pos[w.current()] = 0
}

func (w Words) current() int {
	l := len(w.pos)
	if l == 0 {
		return 0
	}
	return l - 1
}
