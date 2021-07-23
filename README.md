# Chat Registration System

A simple chat system for built for maya's livestream.
Provides a simple API for user authentication, and chat functions.
Frontend & backend code stored here.

## Auth API Documentation

Most API functions will return JSON in the following format:

`status`: either `ok` if action succeeded, or `fail` otherwise.

`reason`: More info about why the action failed.

### Register & Login:

`POST /api/register` with JSON body values of: `name`, `pin`, `pronouns`.

Will return JSON with `status` and `reason`.

`POST /api/login` with JSON body values of: `name`, `pin`.

Will return JSON with `status` and `reason`.

Will set a private cookie named `token` which is used for authentication.

### Change User Information

User information such as name, pin, and pronouns, can be changed currently one at a time.

`POST /api/change` with JSON body values of: `name`, `changed_event`, `new_event`.

`name` the user's current username. used for authentication.

`changed_event` which event to change. value can be one of: `Name`, `Pin`, `Pronouns`.

`new_event` the new value for the changed event.

User is authenticated via token.

### Check if User is Still Logged in

Instead of having to save the pin and re-login every time to check wether they're logged in, you can just check via the token.

`GET /api/token/<name>` where `<name>` is the current username.

Will return JSON with `status` and `reason`.

### Logout

This API will remove the cookie from the client, as well as invalidating the token serverside.

`POST /api/logout` with JSON body values of: `name`.

Will use the current token as authentication.

Will return JSON with `status` and `reason`.

### Get Info About A User

This API will return info about a user on success.

`GET /api/users/<name>`

On success returns JSON in format:

	`status`: `ok`
	`user`:
		`name`: user's name
		`pronouns`: user's pronouns
		`role`: the users role, one of either `Normal`, `Moderator`, or `Admin

eg:

```
{
	status: "ok",
	user: {
		name: "example",
		pronouns: "they/them",
		role: "Normal",
	},
}
```

## Chat API Documentation

`POST /api/message/send {"name":"username","body":"message body"}` Post a message with JSON body values of: `name` & `body`

Will return JSON with `status` and `reason`.

`GET /api/message/messages.json` Returns a json file of all the messages

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
	- [x] Use unix timestamp for date
	- [ ] Create `chat::delete_message()`
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
- [ ] Some form of plural support?
- [ ] User management (banning, etc.)
	- [x] User roles (admin, mod, etc.)
	- [ ] Commands to affect users
- [ ] Blacklist words from chat/names
- [ ] More advanced chat features
	- [x] Different types of message events? eg. default, announcement, command
	- [ ] Types will display differently? eg. announcements pinned to top?
	- [ ] Have different commands?
	- [ ] Emote support?
