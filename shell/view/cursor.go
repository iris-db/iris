package view

// Cursor keeps track of the current words on the STDIN line.
type Cursor struct {
	chars []rune
	pos   int
}

// NewCursor creates a new Cursor.
func NewCursor() *Cursor {
	return &Cursor{
		chars: []rune{},
		pos:   0,
	}
}

func (c *Cursor) PushChar(char rune) {
	c.chars = append(c.chars, char)
	c.pos += 1
	for i := range Sequences {
		Sequences[i].Read(char)
	}
}

func (c *Cursor) Reset() {
	c.chars = []rune{}
	c.pos = 0
}

func (c *Cursor) RemoveChar() {
	c.chars = c.chars[:len(c.chars)-1]
	c.pos -= 1
}

func (c *Cursor) Pos() int {
	return c.pos
}

func (c *Cursor) GetWordDeleteLength() int {
	var lens []int
	var currentLen int

	for i, char := range c.chars {
		if char == ' ' || i == len(c.chars)-1 {
			lens = append(lens, currentLen)
			currentLen = 0
			continue
		}
		currentLen += 1
	}

	var reducedLens []int

	for _, v := range lens {
		if v > 0 {
			reducedLens = append(reducedLens, v+1)
			continue
		}
		if reducedLens != nil {
			reducedLens[len(reducedLens)-1] += 1
		}
	}

	if reducedLens == nil {
		return 0
	}

	return reducedLens[len(reducedLens)-1]
}
