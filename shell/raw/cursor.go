package raw

import "fmt"

// cursor keeps track of the current words on the STDIN line.
type cursor struct {
	chars []string
}

// newCursor creates a new cursor.
func newCursor() *cursor {
	return &cursor{
		chars: []string{},
	}
}

func (c *cursor) PushChar(char string) {
	c.chars = append(c.chars, char)
}

func (c *cursor) RemoveChar() {
	c.chars = c.chars[:len(c.chars)-1]
}

type cursorDirection string

const (
	RIGHT cursorDirection = "\033[C"
	LEFT                  = "\033[D"
	UP                    = "\033[A"
	DOWN                  = "\033[;H"
)

func (c *cursor) Move(d cursorDirection) {
	fmt.Print(d)
}

func (c *cursor) GetWordDeleteLength() int {
	var lens []int
	var currentLen int

	for i, char := range c.chars {
		if char == " " || i == len(c.chars)-1 {
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
