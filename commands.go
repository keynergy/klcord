package main

import (
	"fmt"
	"strings"
)

func LayoutCommand(args []string) string {
	name := strings.Join(args, " ")
	name = strings.ToLower(name)
	l, ok := Layouts[name]
	if !ok {
		return "Can't find that layout."
	}
	response := fmt.Sprintf("__**%s**__\n", l.Name)
	response += fmt.Sprintf("- Created by *%s*\n", l.Creator)
	if l.Modified != "" {
		response += fmt.Sprintf("- Mod of *%s*\n", l.Modified)
	}
	response += "```\n"
	response += strings.Join(l.Keys[0], " ") + "\n"
	response += strings.Join(l.Keys[1], " ") + "\n"
	response += strings.Join(l.Keys[2], " ") + "\n```"
	return response
}
