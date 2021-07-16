# Registration api system
`POST /api/register/<name>/<pin>` Register the username with the pin provided if it doesn't already exist

`GET /api/users/<name>` Check if the user exists

`GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches

`POST /api/register/<name>/<pin>/<new-name>/<new-pin>` Change a users pin/name
