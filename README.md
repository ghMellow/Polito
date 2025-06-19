# Exam #2025: "Gioco della Sfortuna"
## Student: s338680 Termine Nicolò

## React Client Application Routes

- Route `/`: homepage che permette all'utente di interagire con le funzionalità dell'applicativo (info utente, login, regole, cronologia partite e giocare)
- Route `/login`: form di autenticazione per utenti registrati
- Route `/rules`: pagina con le regole e spiegazione del gioco
- Route `/history`: cronologia delle partite completate (richiede autenticazione)
- Route `/history/:gameId`: dettagli di una specifica partita dalla cronologia (richiede autenticazione)
- Route `/game`: interfaccia di gioco
- Route `/summary`: riepilogo della partita appena completata
- Route `*`: pagina 404 per route non esistenti

## API Server

- **POST** `/api/auth/login`
  - Request body: `{ email: string, password: string }`
  - Response: User object `{ id, username, email }` o errore 401
  - Autentica l'utente e crea una sessione

- **GET** `/api/auth/session`
  - No parameters
  - Response: ritorna l'oggetto User se autenticato o errore 401
  - Verifica lo stato della sessione corrente

- **DELETE** `/api/auth/logout`
  - No parameters
  - Response: risponde con stato 200
  - Termina la sessione dell'utente

- **POST** `/api/games/new`
  - Session: utilizza la sessione per reperire user ID se presente altrimenti assegna valore 0 ossia utente anonimo
  - Response: `{ gameId, status, total_cards, correct_guesses, wrong_guesses, cards: [{ id, text, image_path, misfortune_index }] }`
  - Crea una nuova partita con 3 carte iniziali casuali

- **POST** `/api/games/:id/round`
  - URL param: `id` (game ID)
  - Response: `{ roundNumber, card: { id, text, image_path }, timeout }`
  - Genera una nuova carta per il round corrente (non restituisce il misfortune_index della carta)

- **POST** `/api/games/:id/guess`
  - URL param: `id` (game ID)
  - Request body: `{ cardId: number, position: number, roundNumber: number }`
  - Response: `{ correct, correctPosition, timeExpired, message, game: { gameId, status, total_cards, correct_guesses, wrong_guesses, cards: [{ id, text, image_path, misfortune_index }] } }`
  - Valuta la posizione scelta dall'utente, aggiorna e ritorna lo stato della partita tra cui le carte possedute dall'utente e perciò l'eventuale indovinata (misfortune_index presente).

- **GET** `/api/users/profile`
  - Richiede autenticazione, recupera id utente dalla sessione.
  - Response: `{ history: [{ id, status, total_cards, wrong_guesses, correct_guesses, created_at, cards: [{ id, text, image_path, misfortune_index, round_number, won, initial_card }] }] }`
  - Recupera i dati del profilo e la cronologia delle partite dell'utente loggato

- **GET** `/api/cards/image/:path`
  - URL param: `path` (percorso dell'immagine)
  - Response: File immagine o 404 error
  - Serve le immagini delle carte dalla cartella public/images

## Database Tables

- Table `users` - contiene id, username, email, password (hash), salt per l'autenticazione degli utenti registrati
- Table `cards` - contiene id, text, image_path, misfortune_index, category per tutte le carte del gioco
- Table `games` - contiene id, user_id, status, total_cards, wrong_guesses, correct_guesses, created_at per tracciare le partite e i loro stati (in_progress, won, lost)
- Table `game_cards` - contiene id, game_id, card_id, round_number, won, initial_card, created_at per collegare le carte alle partite e tracciare i risultati dei round

## Main React Components

- `DefaultLayout (in DefaultLayout.jsx)`: definisce il layout principale dell'applicazione con navbar, container per messaggi e Outlet per il rendering delle pagine nested
- `Home (in Home.jsx)`: homepage dell'applicazione mostra le informazioni del profilo utente, gestisce utenti loggati e ospiti, pulsanti per sezione regole e gioco
- `GamesHistory (in GamesHistory.jsx)`: cronologia delle partite completate dall'utente e link ai dettagli (richiede login)
- `GameDetails (in GameDetails.jsx)`: dettagli completi di una partita specifica con tutte le carte e risultati dei round (richiede login)
- `Game (in Game.jsx)`: componente che gestisce le meccaniche di gioco, visualizza timer, la carta target e le carte del giocatore
- `GameSummary (in GameSummary.jsx)`: riepilogo della partita conclusa, mostra le carte collezionate e pulsanti per nuova partita e torna alla home
- `LoginForm (in AuthComponents.jsx)`: form di autenticazione con validazione e gestione errori di input
- `LogoutButton (in AuthComponents.jsx)`: bottone che lancia handler di logout usato nelle altre schermate
- `UserAvatar (in UserAvatar.jsx)`: componente che mostra l'immagine utente. Per semplicità di sviluppo questa è uguale per tutti, utenti loggati e ospiti.

## Screenshot

![Screenshot](./img/screenshot.jpg)

## Users Credentials

- mellow@gmail.com, Mellow
- s338680@studenti.polito.it, Password