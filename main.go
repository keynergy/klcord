package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"syscall"

	"github.com/bwmarrin/discordgo"
)

type Layout struct {
	Name string
	Creator string
	Modified string
	Keys [][]string
}

var Layouts map[string]Layout
var LayoutNames []string

func loadlayout(f string) Layout {
	var l Layout
	b, err := ioutil.ReadFile(f)
	if err != nil {
		panic(err)
	}

	s := string(b)
	lines := strings.Split(s, "\n")
	l.Name = strings.TrimSpace(lines[0])
	l.Keys = make([][]string, 3)
	keys := lines[1:4]
	for line := range keys {
		separated := true
		for _, rune := range keys[line] {
			c := string(rune)
			c = strings.ToLower(c)
			if c == " " {
				separated = true
				continue
			} else if !separated {
				continue
			} else {
				separated = false
				l.Keys[line] = append(l.Keys[line], c)
			}
		}
	}
	if len(lines) <= 4 {
		fmt.Println("Warning: missing creator in layout", l.Name)
		l.Creator = "Unknown"
	} else {
		creator := strings.TrimSpace(lines[4])
		l.Creator = creator 
	}
	if len(lines) > 5 {
		modified := strings.TrimSpace(lines[5])
		if modified != "" {
			l.Modified = modified
		}
	}
	return l
}

func getlayouts() {
	Layouts = make(map[string]Layout)
	path := "layouts"
	dir, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	files, _ := dir.Readdirnames(0)
	for _, f := range files {
		l := loadlayout(filepath.Join(path, f))
		if !strings.HasPrefix(f, "_") {
			Layouts[strings.ToLower(l.Name)] = l
			LayoutNames = append(LayoutNames, l.Name)
		}
	}
}

func token() string {
	b, err := ioutil.ReadFile("token")
	if err != nil {
		panic(err)
	}
	s := string(b)
	s = strings.TrimSpace(s)
	return s 
}

func main() {
	dg, err := discordgo.New("Bot " + token())
	
	if err != nil {
		fmt.Println("error creating Discord session,", err)
		os.Exit(1)
	}
	getlayouts()
	dg.AddHandler(MessageCreateHandler)
	dg.Identify.Intents = discordgo.IntentsGuildMessages

	err = dg.Open()
	if err != nil {
		fmt.Println("error opening connection,", err)
		return
	}

	// Wait here until CTRL-C or other term signal is received.
	fmt.Println("Bot is now running.  Press CTRL-C to exit.")
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	// Cleanly close down the Discord session.
	dg.Close()
}
