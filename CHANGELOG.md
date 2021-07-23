## 0.6.0
- Remove deprecated API
- `/api/register` & `/api/login` now use JSON data
- Changing user info now uses Enum (Name, Pin, or Pronouns)

### 0.5.2
- When changing a username, it now deletes the previous user instead of creating a new one
- Database functions should now use some error handling
- Functions use `db_read_user()` instead of reading in the whole database

### 0.5.1
- `/api/logout` API to delete session token
- Add basic support for different message types
- Messages now use unix timestamps
	- Backend finds timestamp

## 0.5.0
- Most actions should now fail on a NULL token
- Cookie should now expire after a week
- Use sled database instead of json file to store users
- Now use `GET /api/token/<name>` to validate a users session token

## 0.4.0
- Serve frontend code
- Set cookie for token
- Basic messaging functionality
- Return JSON for all http requests
- License added (CNPLv6+)

## 0.3.0
- Add todo and other info to README
- Add chat core

## 0.2.0
- Basic functionality added
- Program is split into multiple files
