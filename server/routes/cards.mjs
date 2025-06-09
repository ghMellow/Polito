import express from 'express';
import { dbPromise } from '../db/db.mjs';
import { getCard } from '../db/dao/cards-dao.mjs';

// Route /api/cards/
const router = express.Router();

router.get('/image/:path(*)', (req, res) => {
  const imagePath = req.params.path;
  const fullPath = `public/images/${imagePath}`;
  
  res.sendFile(fullPath, { root: '.' }, (err) => {
    if (err) {
      console.error('Errore nel servire l\'immagine:', err);
      res.status(404).json({ error: 'Immagine non trovata' });
    }
  });
});

export default router;