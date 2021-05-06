package raw

import "testing"

func TestCursor_PushChar(t *testing.T) {

	tests := map[string]struct {
		str  string
		want int
	}{
		"no trailing spaces": {
			str:  "Insert-Node -Data '{}'  -LogAll",
			want: 7,
		},
		"trailing spaces": {
			str:  "Insert-Node -Data '{}'  -LogAll   ",
			want: 10,
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			c := newCursor()

			for _, char := range test.str {
				c.PushChar(string(char))
			}

			if got := c.GetWordDeleteLength(); got != test.want {
				t.Fatalf("got %d; want %d", got, test.want)
			}
		})
	}
}
