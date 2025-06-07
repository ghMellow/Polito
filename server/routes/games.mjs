import express from 'express';
import dayjs from 'dayjs';
import { param, check, validationResult } from 'express-validator';
import { dbPromise } from '../db/db.mjs';
import { getRandomCards, getCard } from '../db/dao/cards-dao.mjs';
import {
  createGame,
  getGame,
  getUserGamesInProgress,
  updateGameStatus,
  incrementWrongGuesses,
  incrementCorrectGuesses,
  deleteGame
} from '../db/dao/games-dao.mjs';
import {
  addRoundCard,
  getGameWonCards,
  getGameUsedCards,
  getNextRoundNumber
} from '../db/dao/game-cards-dao.mjs';


// Route /api/games/
const router = express.Router();

// POST /api/games - Create new game
router.post('/', async (req, res) => {
  try {
    const db = await dbPromise;
    const userId = req.isAuthenticated() ? req.user.id : 0; // per utenti anonimi

    // Verifica se l'utente ha già una partita in corso e la elimina
    const inProgresGames = await getUserGamesInProgress(db, userId);
    console.log(userId, 'Current games for user:', inProgresGames);
    if (inProgresGames.length > 0) {
      for (const currentGameInProgres of inProgresGames) {
          await deleteGame(db, currentGameInProgres.id);
      }
    }

    // Crea nuova partita
    const created_at = dayjs().format();
        console.log('new game created_at:', created_at);

    const gameId = await createGame(db, userId, created_at);
        console.log('new gat:', created_at);

    // Genera 3 carte iniziali casuali
    const limit = 3;
    const initialCards = await getRandomCards(db, limit);
    const roundNumber = null;
    const won = true;
    const initialCard = true;
    console.log('new game created_at:', created_at);
    

    for (const card of initialCards) {
      await addRoundCard(db, gameId, card.id, roundNumber, won, initialCard, created_at);
    }
    
    // Recupera le carte con tutti i dettagli per inviarle al client
    const gameCards = await getGameWonCards(db, gameId);
console.log('Game cards after creation:');

    res.status(201).json({
      gameId: gameId,
      status: 'in_progress',
      total_cards: 3,
      correct_guesses: 0,
      wrong_guesses: 0,
      cards: gameCards
    });

  } catch (error) {
    console.error('Errore nella creazione partita:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

router.post('/:id/round', [
  param('id').isInt({ min: 1 }).withMessage('Game ID must be a positive integer')
], async (req, res) => {
  console.log('POST /api/games/:id/round params:', req.params);
  console.log('POST /api/games/:id/round body:', req.body);
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    console.log('Validation errors:', errors.array());
    return res.status(422).json({ errors: errors.array() });
  }

  try {
    const db = await dbPromise;
    const gameId = req.params.id;

    const game = await getGame(db, gameId);
    if (game.error) {
      return res.status(404).json(game);
    }
    if (game.status !== 'in_progress') {
      return res.status(400).json({ error: 'Game is not in progress' });
    }
    if (req.isAuthenticated() && game.user_id !== req.user.id) {
      return res.status(403).json({ error: 'Access denied to this game' });
    }

    // Per utenti anonimi: solo 1 round (demo)
    const roundNumber = await getNextRoundNumber(db, gameId);
    console.log('Current round number:', roundNumber);    
    if (!req.isAuthenticated()) {
      if (roundNumber > 1) {
        return res.status(400).json({
          error: 'Demo game allows only one round. Please register to play full games.'
        });
      }
    }

    // Ottieni carte già usate per evitare duplicati
    const usedCardIds = await getGameUsedCards(db, gameId);

    // Genera carta casuale non ancora usata
    const limit = 1;
    const hideMisfortune = true;
    const newCards = await getRandomCards(db, limit, hideMisfortune, usedCardIds);
    if (newCards.length === 0) {
      return res.status(400).json({ error: 'No more cards available' });
    }

    const roundCard = newCards[0];

    res.json({
      roundNumber: roundNumber,
      card: roundCard,
      timeout: 30
    });

  } catch (error) {
    console.error('Errore nella generazione round:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

router.post('/:id/guess', [
  param('id').isInt({ min: 1 }).withMessage('Game ID must be a positive integer'),
  check('cardId').isInt({ min: 1 }).withMessage('Card ID must be a positive integer'),
  check('position').isInt({ min: -1, max: 6 }).withMessage('Position must be between 0 and 6'),
  check('roundNumber').isInt({ min: 1 }).withMessage('Round number must be positive')
], async (req, res) => {
  console.log('POST /api/games/:id/guess params:', req.params);
  console.log('POST /api/games/:id/guess body:', req.body);
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    console.log('Validation errors:', errors.array());
    return res.status(422).json({ errors: errors.array() });
  }

  try {
    const db = await dbPromise;
    const gameId = req.params.id;
    const { cardId, position, roundNumber } = req.body;

    const game = await getGame(db, gameId);
    if (game.error) {
      return res.status(404).json(game);
    }
    if (game.status !== 'in_progress') {
      return res.status(400).json({ error: 'Game is not in progress' });
    }
    if (req.isAuthenticated() && game.user_id !== req.user.id) {
      return res.status(403).json({ error: 'Access denied to this game' });
    }

    // Ottieni la carta giocata
    const roundCard = await getCard(db, cardId);
    if (roundCard.error) {
      return res.status(404).json({ error: 'Card not found' });
    }

    // Verifica timeout di 30 secondi dal created_at della carta
    const currentTime = dayjs();
    const cardCreatedTime = dayjs(roundCard.created_at);
    const timeDifference = currentTime.diff(cardCreatedTime, 'second');

    // Ottieni le carte possedute PRIMA di qualsiasi modifica
    const ownedCards = await getGameWonCards(db, gameId);
    const sortedCards = ownedCards.sort((a, b) => a.misfortune_index - b.misfortune_index);
    const correctPosition = findCorrectPosition(roundCard.misfortune_index, sortedCards);
    const isCorrect = position === correctPosition;

    const initialCard = false;
    let gameStatus = 'in_progress';
    let message = '';

    // Gestisci timeout
    if (timeDifference > 30) {
      // Tempo scaduto - carta persa
      const won = false;
      await addRoundCard(db, gameId, cardId, roundNumber, won, initialCard);
      await incrementWrongGuesses(db, gameId);
      
      message = 'Time expired! You lost this card.';

      // Verifica sconfitta (3 errori)
      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, gameStatus, game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }
    } else if (isCorrect) {
      // Carta indovinata correttamente
      await addRoundCard(db, gameId, cardId, roundNumber, true, initialCard);
      await incrementCorrectGuesses(db, gameId);
      
      message = 'Congratulations! You guessed correctly!';

      // Verifica vittoria (6 carte totali)
      if ((ownedCards.length + 1) >= 6) {
        gameStatus = 'won';
        await updateGameStatus(db, gameId, 'won', 6);
        message = 'You won the game! Congratulations!';
      }
    } else {
      // Carta sbagliata
      await addRoundCard(db, gameId, cardId, roundNumber, false, initialCard);
      await incrementWrongGuesses(db, gameId);
      
      message = 'Wrong guess! Try again.';

      // Verifica sconfitta (3 errori)
      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, 'lost', game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }
    }

    // Recupera stato aggiornato DOPO tutte le modifiche
    const updatedGame = await getGame(db, gameId);
    const updatedCards = await getGameWonCards(db, gameId);

    let cardToSend = roundCard;
    if (timeDifference > 30 || !isCorrect) {
      cardToSend = {
        id: roundCard.id,
        text: roundCard.text,
        image_path: roundCard.image_path,
        category: roundCard.category
        // misfortune_index volutamente omesso
      };
    }
    console.log("\n> > > roundCard: roundCard: \n\n", cardToSend);

    res.json({
      correct: isCorrect,
      correctPosition: correctPosition,
      timeExpired: timeDifference > 30,
      card: cardToSend,
      message: message,
      game: {
        id: updatedGame.id,
        status: updatedGame.status,
        total_cards: updatedGame.total_cards,
        correct_guesses: updatedGame.correct_guesses,
        wrong_guesses: updatedGame.wrong_guesses,
        cards: updatedCards
      }
    });

  } catch (error) {
    console.error('Errore nella valutazione guess:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

function findCorrectPosition(misfortuneIndex, sortedCards) {
  for (let i = 0; i < sortedCards.length; i++) {
    if (misfortuneIndex < sortedCards[i].misfortune_index) {
      return i;
    }
  }
  return sortedCards.length; // Inserire alla fine
}

export default router;