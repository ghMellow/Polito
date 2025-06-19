import express from 'express';
import dayjs from 'dayjs';
import { param, check, validationResult } from 'express-validator';
import { dbPromise } from '../db/db.mjs';
import { getAllCards, getCard } from '../db/dao/cards-dao.mjs';
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
  getCurrentRoundNumber
} from '../db/dao/game-cards-dao.mjs';


// Route /api/games/
const router = express.Router();

router.post('/new', async (req, res) => {
  try {
    const db = await dbPromise;
    const userId = req.isAuthenticated() ? req.user.id : 0; // per utenti anonimi

    const inProgresGames = await getUserGamesInProgress(db, userId);
    if (inProgresGames.length > 0) {
      for (const currentGameInProgres of inProgresGames) {
        await deleteGame(db, currentGameInProgres.id);
      }
    }

    const created_at = dayjs().format();
    const gameId = await createGame(db, userId, created_at);

    const limit = 3;
    const roundNumber = null;
    const won = true;
    const initialCard = true;
    const initialCards = await getRandomCards(db, limit);
    initialCards.sort((a, b) => a.misfortune_index - b.misfortune_index);
    for (const card of initialCards) {
      await addRoundCard(db, gameId, card.id, roundNumber, won, initialCard, created_at);
    }

    res.status(201).json({
      gameId: gameId,
      status: 'in_progress',
      total_cards: 3,
      correct_guesses: 0,
      wrong_guesses: 0,
      cards: initialCards
    });

  } catch (error) {
    console.error('Errore nella creazione partita:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});


export const getRandomCards = async (db, limit, hideMisfortune = false, excludeIds = []) => {
  try {
    const allCards = await getAllCards(db, hideMisfortune);
    const filteredCards = allCards.filter(card => !excludeIds.includes(card.id));
    return filteredCards
      .sort(() => Math.random() - 0.5)
      .slice(0, limit);
  } catch (error) {
    throw error;
  }
};

router.post('/:id/round', [
  param('id').isInt({ min: 1 }).withMessage('Game ID must be a positive integer')
], async (req, res) => {
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
    const roundNumber = await getCurrentRoundNumber(db, gameId);
    if (!req.isAuthenticated() && roundNumber > 1) {
      return res.status(400).json({
        error: 'Demo game allows only one round. Please register to play full games.'
      });
    }

    const limit = 1;
    const hideMisfortune = true;
    const usedCardIds = await getGameUsedCards(db, gameId);
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

    const roundCard = await getCard(db, cardId);
    if (roundCard.error) {
      return res.status(404).json({ error: 'Card not found' });
    }

    const currentTime = dayjs();
    const cardCreatedTime = dayjs(roundCard.created_at);
    const timeDifference = currentTime.diff(cardCreatedTime, 'second');

    const ownedCards = await getGameWonCards(db, gameId);
    const correctPosition = findCorrectPosition(roundCard.misfortune_index, ownedCards);
    const isCorrect = position === correctPosition;

    const initialCard = false;
    let gameStatus = 'in_progress';
    let message = '';
    let won = false;
    if (timeDifference > 30) {
      won = false;
      await addRoundCard(db, gameId, cardId, roundNumber, won, initialCard);
      await incrementWrongGuesses(db, gameId);

      message = 'Time expired! You lost this card.';

      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, gameStatus, game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }
    } else if (isCorrect) {
      won = true;
      await addRoundCard(db, gameId, cardId, roundNumber, won, initialCard);
      await incrementCorrectGuesses(db, gameId);

      message = 'Congratulations! You guessed correctly!';

      if ((ownedCards.length + 1) >= 6) {
        gameStatus = 'won';
        await updateGameStatus(db, gameId, 'won', 6);
        message = 'You won the game! Congratulations!';
      }
    } else {
      won = false;
      await addRoundCard(db, gameId, cardId, roundNumber, won, initialCard);
      await incrementWrongGuesses(db, gameId);

      message = 'Wrong guess! Try again.';

      if (game.wrong_guesses + 1 >= 3) {
        gameStatus = 'lost';
        await updateGameStatus(db, gameId, 'lost', game.total_cards);
        message = 'Game over! You made too many wrong guesses.';
      }
    }

    // Aggiorno stato dopo le modifiche
    const updatedGame = await getGame(db, gameId);
    const updatedCards = await getGameWonCards(db, gameId);

    res.json({
      correct: isCorrect,
      correctPosition: correctPosition,
      timeExpired: timeDifference > 30,
      message: message,
      game: {
        gameId: updatedGame.id,
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
  return sortedCards.length;
}

export default router;