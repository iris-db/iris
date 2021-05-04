package lib

import (
	"fmt"
	"os"
)

// ExitErr prints an error and exits the program with a status code of 1.
func ExitErr(err error) {
	fmt.Println(err.Error())
	os.Exit(1)
}

// PrintDivider prints a horizontal divider.
func PrintDivider() {
	fmt.Println("============================")
}
