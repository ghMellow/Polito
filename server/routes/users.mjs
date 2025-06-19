import express from 'express';
import { isLoggedIn } from './auth-middleware.mjs';
import { dbPromise } from '../db/db.mjs';
import { getUserProfile } from '../db/dao/game-cards-dao.mjs';

// Route /api/users/
const router = express.Router();

router.get('/profile', isLoggedIn, async (req, res) => {
  try {
    const db = await dbPromise;
    const user = req.user;

    const gameHistory = await getUserProfile(db, user.id);

    res.json({
      history: gameHistory
    });

  } catch (error) {
    console.error('Errore nel recupero profilo utente:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

export default router;