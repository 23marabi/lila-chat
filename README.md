# Chat Registration System

The basic backend code needed to register & login to a chat system (to be built).
Send it the unhashed username and pin, and it'll store it in the `users.json` file with the pin hashed with SHA1.

## API Documentation

`POST /api/register/<name>/<pin>` Register the username with the pin provided if it doesn't already exist

`GET /api/users/<name>` Check if the user exists

`GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches

`POST /api/users/change/<name>/<pin>/<new-name>/<new-pin>` Change a users pin/name

## Chat Planning

Clientside js will register & check login of users, if login is correct will ask for a random token.
Backend will generate token, store it, and then send it to the client to set as a cookie.
Whenever user sends a message, client will send message & token and backend will check if token matches.

## To-Do:

- [x] Basic auth api
- [ ] Basic messaging system
- [ ] Token generation & storage
- [x] Pronouns
