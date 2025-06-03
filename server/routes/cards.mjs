import express from 'express';
import { dbPromise } from '../db/db.mjs';

const router = express.Router();

// Endpoint per ottenere una carta specifica
router.get('/:id', async (req, res) => {
  try {
    const db = await dbPromise;
    const card = await db.get('SELECT * FROM cards WHERE id = ?', [req.params.id]);
    
    if (!card) {
      return res.status(404).json({ error: 'Carta non trovata' });
    }
    
    const cardWithImageUrl = {
      ...card,
      image_url: `${req.protocol}://${req.get('host')}/images/${card.image_path}`
    };
    
    res.json(cardWithImageUrl);
  } catch (error) {
    console.error('Errore nel recuperare la carta:', error);
    res.status(500).json({ error: 'Errore interno del server' });
  }
});

router.get('/image/:path(*)', (req, res) => {
  const imagePath = req.params.path;
  const fullPath = `public/images/${imagePath}`;
  
  // Invia il file immagine
  res.sendFile(fullPath, { root: '.' }, (err) => {
    if (err) {
      console.error('Errore nel servire l\'immagine:', err);
      res.status(404).json({ error: 'Immagine non trovata' });
    }
  });
});

export default router;