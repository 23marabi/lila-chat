# Chat Registration System

A simple chat system for built for maya's livestream.
Provides a simple API for user authentication, and chat functions.
Frontend & backend code stored here.

## Implemented Features:

- [x] Basic auth API
- [x] Return json instead of string
	- "status" shows wether request was succesful or not, either "ok" or "fail"
	- "reason" is for more details, mainly just for debugging?
- [x] Basic messaging system
	- [x] Finish up `chat::create_message()`
	- [x] Create `chat::fetch_messages()`
	- [x] Use unix timestamp for date
- [x] Switch to using sled database to store users
	- [x] Error handling
- [x] Token generation & storage
	- [x] Sets cookie
	- [x] Store token in json
	- [x] Have cookie expire
	- [x] Remove old cookie
	- [x] Use token for most stuff
	- [x] Logout API
	- [x] Fail on NULL token
- [x] Pronouns
	- [x] Set pronouns
	- [x] Change pronouns
- [x] make changed_event Enum, use token instead of pin

## To-Do:

- [x] Better messaging system
	- [ ] Use websockets for chat [issue#3](https://git.lavender.software/erin/lila-chat/issues/3)
	- [ ] Create `chat::delete_message()`
	- [ ] Display pronouns [issue#4](https://git.lavender.software/erin/lila-chat/issues/4)
- [ ] Various database improvements [issue#1](https://git.lavender.software/erin/lila-chat/issues/1)
	- [ ] Improve error handling on write functions
	- [ ] Allow for asyncronous reading/writing
- [ ] Some form of plural support?
- [ ] User management (banning, etc.)
	- [x] User roles (admin, mod, etc.)
	- [ ] Commands to affect users [issue#2](https://git.lavender.software/erin/lila-chat/issues/2)
- [ ] Blacklist words from chat/names
- [ ] More advanced chat features
	- [x] Different types of message events? eg. default, announcement, command
	- [ ] Types will display differently? eg. announcements pinned to top?
	- [ ] Have different commands?
	- [ ] Emote support?
