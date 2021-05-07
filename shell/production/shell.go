package main

import (
	"fmt"
	"github.com/Im-Stevemmmmm/keyboard"
)

func main() {
	fmt.Println("Iris shell version v.1.0.0")
	fmt.Println("CONNPOOL Connecting to local database")
	fmt.Print("localhost> ")

	if err := keyboard.Open(); err != nil {
		panic(err)
	}
	defer func() {
		_ = keyboard.Close()
	}()

	for {

		char, key, err := keyboard.GetKey()
		if err != nil {
			panic(err)
		}

		//fmt.Printf("%X\r", key)
		fmt.Printf("%c", char)

		if key == keyboard.KeyCtrlC {
			fmt.Println()
			break
		}
	}
}
