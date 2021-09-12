package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestLayoutCommand(t *testing.T) {
	getlayouts()
	cases := []string{"colemak", "cOlEmAk", "colemak dh", "", "sraietdsnrt"}
	for _, c := range cases {
		fmt.Println(c)
		fmt.Println(LayoutCommand(strings.Split(c, " ")))
	}
}
