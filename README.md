# Chat Registration System

The basic backend code needed to register & login to a chat system (to be built).
Send it the unhashed username and pin, and it'll store it in the `users.json` file with the pin hashed with SHA1.

## API Documentation

`POST /api/register/<name>/<pin>` Register the username with the pin provided if it doesn't already exist

`GET /api/users/<name>` Check if the user exists

`GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches

`POST /api/users/change/<name>/<pin>/<new-name>/<new-pin>` Change a users pin/name
