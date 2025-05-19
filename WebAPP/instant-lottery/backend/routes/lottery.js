// File: routes/lottery.js
import express from 'express';
import { 
  initLottoTables,
  createEstrazione,
  getLatestEstrazione,
  createPuntata,
  verificaVincite,
  getPuntateUtente 
} from '../db/lottery-dao.js';
import { getUser, updateUserPoints } from '../db/users-dao.js';

const router = express.Router();

// Inizializzazione DB (può essere chiamato all'avvio dell'app)
router.post('/init', async (req, res) => {
  try {
    await initLottoTables(req.app.locals.db);
    res.status(200).json({ message: 'Tabelle della lotteria inizializzate con successo' });
  } catch (err) {
    console.error('Errore durante l\'inizializzazione delle tabelle:', err);
    res.status(500).json({ error: 'Errore durante l\'inizializzazione del database' });
  }
});

// GET /api/lottery/estrazioni/latest - Ottieni l'ultima estrazione
router.get('/estrazioni/latest', async (req, res) => {
  try {
    const estrazione = await getLatestEstrazione(req.app.locals.db);
    if (!estrazione) {
      return res.status(404).json({ error: 'Nessuna estrazione trovata' });
    }
    res.json(estrazione);
  } catch (err) {
    console.error('Errore nel recupero dell\'ultima estrazione:', err);
    res.status(500).json({ error: 'Errore nel recupero dell\'ultima estrazione' });
  }
});

// POST /api/v1/lottery/estrazioni - Crea una nuova estrazione
router.post('/estrazioni', async (req, res) => {
    try {
      // Otteniamo l'ultima estrazione
      const ultimaEstrazione = await getLatestEstrazione(req.app.locals.db);
      
      // Se esiste un'ultima estrazione, verifichiamo che siano passati almeno 2 minuti
      if (ultimaEstrazione) {
        const timestampUltimaEstrazione = new Date(ultimaEstrazione.timestamp);
        const adesso = new Date();
        
        // Calcoliamo la differenza in millisecondi
        const differenzaMs = adesso - timestampUltimaEstrazione;
        const differenzaMinuti = differenzaMs / (1000 * 60);
        
        // Se sono passati meno di 2 minuti, restituiamo un errore
        if (differenzaMinuti < 2) {
          const tempoRimanente = Math.ceil(2 - differenzaMinuti);
          return res.status(429).json({ 
            error: 'È necessario attendere tra un\'estrazione e l\'altra',
            message: `Attendi ancora ${tempoRimanente} minuti prima di creare una nuova estrazione`,
            nextAllowedAt: new Date(timestampUltimaEstrazione.getTime() + 2 * 60 * 1000).toISOString()
          });
        }
      }
      
      // Se sono passati più di 2 minuti o non c'è un'estrazione precedente, procediamo
      const estrazione = await createEstrazione(req.app.locals.db);
      res.status(201).json(estrazione);
    } catch (err) {
      console.error('Errore nella creazione dell\'estrazione:', err);
      res.status(500).json({ error: 'Errore nella creazione dell\'estrazione' });
    }
  });

// POST /api/lottery/puntate - Crea una nuova puntata
router.post('/puntate', async (req, res) => {
  const { userId, estrazioneId, numeri } = req.body;

  // Validazione input
  if (!userId || !estrazioneId || !numeri || !Array.isArray(numeri)) {
    return res.status(400).json({ error: 'Dati richiesti mancanti o non validi' });
  }

  // Verifica che i numeri siano da 1 a 3
  if (numeri.length < 1 || numeri.length > 3) {
    return res.status(400).json({ error: 'È possibile puntare da 1 a 3 numeri' });
  }

  // Verifica che i numeri siano compresi tra 1 e 90
  for (const num of numeri) {
    if (num < 1 || num > 90 || !Number.isInteger(num)) {
      return res.status(400).json({ error: 'I numeri devono essere interi compresi tra 1 e 90' });
    }
  }

  // Verifica che non ci siano numeri duplicati
  if (new Set(numeri).size !== numeri.length) {
    return res.status(400).json({ error: 'Non puoi puntare lo stesso numero più volte' });
  }

  try {
    const puntata = await createPuntata(
      req.app.locals.db, 
      userId, 
      estrazioneId, 
      numeri,
      // Passiamo le funzioni necessarie per ottenere e aggiornare l'utente
      (db, userId) => getUser(userId, db),
      (db, userId, points) => updateUserPoints(userId, points, db)
    );
    res.status(201).json(puntata);
  } catch (err) {
    console.error('Errore nella creazione della puntata:', err);
    res.status(400).json({ error: err.message });
  }
});

// POST /api/lottery/estrazioni/:id/verifica - Verifica le vincite di un'estrazione
router.post('/estrazioni/:id/verifica', async (req, res) => {
  const estrazioneId = req.params.id;
  
  try {
    const numPuntateProcessate = await verificaVincite(
      req.app.locals.db, 
      estrazioneId,
      // Passiamo le funzioni necessarie per ottenere e aggiornare l'utente
      (db, userId) => getUser(userId, db),
      (db, userId, points) => updateUserPoints(userId, points, db)
    );
    res.json({ 
      message: 'Verifica vincite completata con successo', 
      puntateProcessate: numPuntateProcessate 
    });
  } catch (err) {
    console.error('Errore nella verifica delle vincite:', err);
    res.status(500).json({ error: err.message });
  }
});

// GET /api/lottery/puntate/user/:userId - Ottieni le puntate di un utente
router.get('/puntate/user/:userId', async (req, res) => {
  const userId = req.params.userId;
  
  try {
    const puntate = await getPuntateUtente(req.app.locals.db, userId);
    res.json(puntate);
  } catch (err) {
    console.error('Errore nel recupero delle puntate dell\'utente:', err);
    res.status(500).json({ error: 'Errore nel recupero delle puntate' });
  }
});

export default router;