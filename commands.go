package main

import "strings"

func LayoutCommand(args []string) string {
	name := strings.Join(args, " ")
	name = strings.ToLower(name)
	l, ok := Layouts[name]
	if !ok {
		return "Can't find that layout."
	}
	response := l.Name + "\n```\n"
	response += strings.Join(l.Keys[0], " ") + "\n"
	response += strings.Join(l.Keys[1], " ") + "\n"
	response += strings.Join(l.Keys[2], " ") + "\n```"
	return response
}
