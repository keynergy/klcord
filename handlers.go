package main

import (
	"strings"

	"github.com/bwmarrin/discordgo"
)

func MessageCreateHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
	// Ignore bot messages
	if m.Author.ID == s.State.User.ID {
		return
	}

	split := strings.Split(m.Content, " ")
	if split[0] == "!layout" {
		response := LayoutCommand(split[1:])
		s.ChannelMessageSend(m.ChannelID, response)
	}
}
