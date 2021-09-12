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
	Keys [][]string
}

var Layouts map[string]Layout

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
	return l
}

func getlayouts() {
	Layouts = make(map[string]Layout)
	b, err := ioutil.ReadFile("layoutsdir")
	if err != nil {
		panic(err)
	}
	path := strings.TrimSpace(string(b))
	dir, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	files, _ := dir.Readdirnames(0)
	for _, f := range files {
		l := loadlayout(filepath.Join(path, f))
		if !strings.HasPrefix(f, "_") {
			Layouts[strings.ToLower(l.Name)] = l		
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
