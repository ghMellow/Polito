import express from 'express';
import dayjs from 'dayjs';
import { check, validationResult } from 'express-validator';
import { dbPromise } from '../db/db.mjs';

// DAO imports
import { getRandomCards, getCard } from '../db/dao/cards-dao.mjs';
import {
  createGame,
  getGame,
  getCurrentGame,
  updateGameStatus,
  incrementWrongGuesses,
  incrementTotalCards
} from '../db/dao/games-dao.mjs';
import {
  addInitialCards,
  addRoundCard,
  getGameWonCards,
  getGameUsedCards,
  getCurrentRoundNumber
} from '../db/dao/game-cards-dao.mjs';


// Route /api/games/
const router = express.Router();

// POST /api/games - Create new game
router.post('/', async (req, res) => {
  try {
    const db = await dbPromise;
    const userId = req.isAuthenticated() ? req.user.id : 0; // 0 per utenti anonimi

    // Verifica se l'utente ha già una partita in corso e la elimina
    const currentGame = await getCurrentGame(db, userId);
    if (currentGame) {
      // Elimina le carte della partita in corso
      await deleteGameCards(db, currentGame.id);
      // Elimina la partita in corso
      await deleteGame(db, currentGame.id);
    }

    // Crea nuova partita
    const gameId = await createGame(db, userId);

    // Genera 3 carte iniziali casuali
    const initialCards = await getRandomCards(db, 3);
    await addInitialCards(db, gameId, initialCards.map(c => c.id));

    // Recupera le carte con tutti i dettagli per inviarle al client
    const gameCards = await getGameWonCards(db, gameId);

    res.status(201).json({
      gameId: gameId,
      status: 'in_progress',
      totalCards: 3,
      wrongGuesses: 0,
      cards: gameCards
    });

  } catch (error) {
    console.error('Errore nella creazione partita:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// GET /api/games/:id - Get game status
router.get('/:id', [
  check('id').isInt({ min: 1 }).withMessage('Game ID must be a positive integer')
], async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    return res.status(422).json({ errors: errors.array() });
  }

  try {
    const db = await dbPromise;
    const gameId = req.params.id;

    const game = await getGame(db, gameId);
    if (game.error) {
      return res.status(404).json(game);
    }

    // Verifica che la partita appartenga all'utente (se autenticato)
    if (req.isAuthenticated() && game.user_id !== req.user.id) {
      return res.status(403).json({ error: 'Access denied to this game' });
    }

    // Recupera le carte vinte
    const cards = await getGameWonCards(db, gameId);

    res.json({
      id: game.id,
      status: game.status,
      totalCards: game.total_cards,
      wrongGuesses: game.wrong_guesses,
      createdAt: game.created_at,
      completedAt: game.completed_at,
      cards: cards
    });

  } catch (error) {
    console.error('Errore nel recupero partita:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

router.post('/:id/round', [
  check('id').isInt({ min: 1 }).withMessage('Game ID must be a positive integer')
], async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
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

    // Verifica che la partita appartenga all'utente (se autenticato)
    if (req.isAuthenticated() && game.user_id !== req.user.id) {
      return res.status(403).json({ error: 'Access denied to this game' });
    }

    // Per utenti anonimi: solo 1 round (demo)
    if (!req.isAuthenticated()) {
      const currentRound = await getCurrentRoundNumber(db, gameId);
      if (currentRound > 1) {
        return res.status(400).json({
          error: 'Demo game allows only one round. Please register to play full games.'
        });
      }
    }

    // Ottieni carte già usate per evitare duplicati
    const usedCardIds = await getGameUsedCards(db, gameId);

    // Genera carta casuale non ancora usata
    limit = 1;
    hideMisfortune = true;
    const newCards = await getRandomCards(db, limit, hideMisfortune, usedCardIds);
    if (newCards.length === 0) {
      return res.status(400).json({ error: 'No more cards available' });
    }

    const roundCard = newCards[0];
    const roundNumber = await getCurrentRoundNumber(db, gameId);

    // Invia solo nome e immagine (NON l'indice di sfortuna!)
    res.json({
      roundNumber: roundNumber,
      card: {
        id: roundCard.id,
        name: roundCard.name,
        image_path: roundCard.image_path
        // misfortune_index volutamente omesso!
      },
      timeout: 30000 // 30 secondi
    });

  } catch (error) {
    console.error('Errore nella generazione round:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

router.post('/:id/guess', [
  check('gameId').isInt({ min: 1 }).withMessage('Game ID must be a positive integer'),
  check('cardId').isInt({ min: 1 }).withMessage('Card ID must be a positive integer'),
  check('position').isInt({ min: 0, max: 6 }).withMessage('Position must be between 0 and 6'),
  check('roundNumber').isInt({ min: 1 }).withMessage('Round number must be positive')
], async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
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

    // Verifica che la partita appartenga all'utente (se autenticato)
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

    if (timeDifference > 30) {
      // Tempo scaduto - carta persa e non mostrata
      await addRoundCard(db, gameId, cardId, roundNumber, false);
      await incrementWrongGuesses(db, gameId);

      let gameStatus = 'in_progress';
      let message = 'Time expired! You lost this card.';

      // Verifica sconfitta (3 errori)
      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, 'lost', game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }

      // Recupera stato aggiornato
      const updatedGame = await getGame(db, gameId);
      const updatedCards = await getGameWonCards(db, gameId);

      return res.json({
        correct: false,
        timeExpired: true,
        message: message,
        game: {
          id: updatedGame.id,
          status: gameStatus,
          totalCards: updatedGame.total_cards,
          wrongGuesses: updatedGame.wrong_guesses,
          cards: updatedCards
        }
      });
    }

    // Ottieni le carte possedute
    const ownedCards = await getGameWonCards(db, gameId);

    // Verifica posizione del guess
    const sortedCards = ownedCards.sort((a, b) => a.misfortune_index - b.misfortune_index);
    const correctPosition = findCorrectPosition(roundCard.misfortune_index, sortedCards);

    const isCorrect = position === correctPosition;

    // Salva risultato del round
    await addRoundCard(db, gameId, cardId, roundNumber, isCorrect);

    let gameStatus = 'in_progress';
    let message = '';

    if (isCorrect) {
      // Carta vinta
      await incrementTotalCards(db, gameId);
      message = 'Congratulations! You guessed correctly!';

      // Verifica vittoria (6 carte totali)
      if (game.total_cards + 1 >= 6) {
        gameStatus = 'won';
        await updateGameStatus(db, gameId, 'won', 6);
        message = 'You won the game! Congratulations!';
      }
    } else {
      // Carta persa
      await incrementWrongGuesses(db, gameId);
      message = 'Wrong guess! Try again.';

      // Verifica sconfitta (3 errori)
      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, 'lost', game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }
    }

    // Recupera stato aggiornato
    const updatedGame = await getGame(db, gameId);
    const updatedCards = await getGameWonCards(db, gameId);

    res.json({
      correct: isCorrect,
      correctPosition: correctPosition,
      timeExpired: false,
      card: roundCard, // Mostra tutti i dettagli solo se indovinato o tempo non scaduto
      message: message,
      game: {
        id: updatedGame.id,
        status: gameStatus,
        totalCards: updatedGame.total_cards,
        wrongGuesses: updatedGame.wrong_guesses,
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