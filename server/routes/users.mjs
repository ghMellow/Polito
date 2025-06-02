import express from 'express';
import { isLoggedIn } from './auth-middleware.mjs';
import { dbPromise } from '../db/db.mjs';
import { getUserById } from '../db/dao/users-dao.mjs';
import { getUserGameHistoryWithCards } from '../db/dao/game-cards-dao.mjs';

// Route /api/users/
const router = express.Router();

router.get('/profile', isLoggedIn, async (req, res) => {
  try {
    const db = await dbPromise;
    const userId = req.user.id;
    
    const user = await getUserById(db, userId);
    if (!user) {
      return res.status(404).json({ error: 'User not found' });
    }
    
    
    const gameHistory = await getUserGameHistoryWithCards(db, userId);
    
    res.json({
      user: {
        username: user.username,
        email: user.email,
      },
      history: gameHistory
    });
    
  } catch (error) {
    console.error('Errore nel recupero profilo utente:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

export default router;