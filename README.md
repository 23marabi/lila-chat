# Chat Registration System

The basic backend code needed to register & login to a chat system (to be built).
Send it the unhashed username and pin, and it'll store it in the database with the pin hashed with SHA1.

## API Documentation

`POST /api/register {"name":"<username>","pin":"<pin>","pronouns":"<pronouns>"}` Register a user if they don't already exist

`POST /api/register/<name>/<pin>/<pronouns>` Register the username with the pin provided if it doesn't already exist
Returns status & reason json.

`GET /api/users/<name>` Check if the user exists
Returns either

`{
	"status": "fail",
	"reason": "user not found",
}`

or

`{
	"status": "ok",
	"user": {
		"name": "<name>",
		"pronouns": "<pronouns>",
	},
}`

`GET /api/token/<name>` Check if the current token matches the user provided

`GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches
Returns status & reason json.

`POST /api/users/change {"name":"<username>","pin":"<pin>","changed_event":"name/pin/pronouns","new_event":"<new name/pin/pronouns>"` Change a users details via a json post.

eg. `POST /api/users/change {"name":"example","pin":"10","changed_event":"name","new_event":"test"` to change the user "example"'s name to "test"

DEPRECATED `POST /api/users/change/<name>/<pin>/<new-name>/<new-pin>` Change a users pin/name
Returns status & reason json.

`POST /api/logout {"name":"<username>"}` to logout a user if the token matches


## Chat Documentation

`POST /api/message/send {"name":"username","body":"message body","date":"yyyy-mm-dd"}` Post a json message.
Returns status & reason json.

`GET /api/message/messages.json` Get a json file of all the messages

## Chat Planning

Clientside js will register & check login of users, if login is correct will ask for a random token.
Backend will generate token, store it, and then send it to the client to set as a cookie.
Whenever user sends a message, client will send message & token and backend will check if token matches.

## To-Do:

- [x] Basic auth API
- [x] Return json instead of string
	- "status" shows wether request was succesful or not, either "ok" or "fail"
	- "reason" is for more details, mainly just for debugging?
- [x] Basic messaging system
	- [x] Finish up `chat::create_message()`
	- [x] Create `chat::fetch_messages()`
	- [ ] Use unix timestamp for date
	- [ ] Create `chat::delete_message()`
- [x] Switch to using sled database to store users
	- [ ] Error handling
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
	- [ ] Change pronouns
	- [ ] Multiple sets of pronouns
- [ ] Some form of plural support?
- [ ] User management (banning, etc.)
- [ ] Blacklist words from chat/names
- [ ] More advanced chat features
	- [ ] Different types of message events? eg. default, announcement, command
	- [ ] Emote support?
