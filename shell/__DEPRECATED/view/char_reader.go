package view

// CharReader reads a character.
type CharReader interface {
	Read(c rune)
}