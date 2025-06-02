# Exam #2025: "Gioco della Sfortuna"
## Student: s338680 Termine Nicolò

## React Client Application Routes

- Route `/`: home page with game instructions and login/register options for new users
- Route `/login`: authentication page for existing users
- Route `/game`: main game interface where users play the "Gioco della Sfortuna"
- Route `/game/:id`: specific game session page with game ID parameter
- Route `/profile`: user profile page showing statistics and game history (protected route)
- Route `/demo`: demo game page for anonymous users (single round only)

## API Server

### Authentication APIs
- POST `/api/auth/login`
  - request body: `{ "username": string, "password": string }`
  - response body: `{ "id": number, "username": string, "created_at": string }`

- GET `/api/auth/session`
  - request parameters: none (uses session cookie)
  - response body: `{ "id": number, "username": string, "created_at": string }` or `{ "error": "Not authenticated" }`

- DELETE `/api/auth/logout`
  - request parameters: none
  - response body: empty (status 200)

### Game APIs
- POST `/api/games`
  - request parameters: none (user ID from session if authenticated)
  - response body: `{ "gameId": number, "status": "in_progress", "totalCards": 3, "wrongGuesses": 0, "cards": Array<Card> }`

- GET `/api/games/:id`
  - request parameters: `id` (game ID as integer)
  - response body: `{ "id": number, "status": string, "totalCards": number, "wrongGuesses": number, "createdAt": string, "completedAt": string, "cards": Array<Card> }`

- POST `/api/games/:id/round`
  - request parameters: `id` (game ID as integer)
  - response body: `{ "roundNumber": number, "card": { "id": number, "name": string, "image_path": string }, "timeout": 30000 }`

- POST `/api/games/:id/guess`
  - request parameters: `id` (game ID as integer)
  - request body: `{ "cardId": number, "position": number, "roundNumber": number }`
  - response body: `{ "correct": boolean, "correctPosition": number, "card": Card, "message": string, "game": GameState }`

### User APIs
- GET `/api/users/profile`
  - request parameters: none (user ID from session)
  - response body: `{ "user": User, "stats": GameStats, "history": Array<GameHistory> }`

- GET `/api/users/history`
  - request parameters: none (user ID from session)
  - response body: `Array<GameHistory>`


## Database Tables

- Table `users` - contains user credentials and account information (id, username, email, password, salt)
- Table `cards` - contains the 50+ misfortune situation cards with their details (id, text, image_path, misfortune_index, category, created_at)
- Table `games` - contains game sessions and their status (id, user_id, status, total_cards, wrong_guesses, created_at, completed_at)
- Table `game_cards` - contains the relationship between games and cards, tracking which cards were played in each game (id, game_id, card_id, round_number, won, initial_card, created_at)

## Main React Components


## Screenshot

![Screenshot 1 - Game in Progress](./img/game-screenshot.jpg)
![Screenshot 2 - User Profile History](./img/profile-screenshot.jpg)

## Users Credentials

- mario.rossi, password123 (user with game history - 3 completed games)
- giulia.verdi, mypassword (new user with no game history)