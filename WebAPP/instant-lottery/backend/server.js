// File: server.js
import express from 'express';
import cors from 'cors';
import { dbPromise, closeDb } from './db/db.js';

import lotteryRouter from './routes/lottery.js';
import usersRouter from './routes/users.js';

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Avviamo il server solo dopo aver inizializzato il DB
dbPromise
  .then((db) => {
    // Rendiamo il DB disponibile per le route
    app.locals.db = db;

    // Routes
    app.use('/api/v1/lottery', lotteryRouter);
    app.use('/api/v1/users', usersRouter);

    // Route di base
    app.get('/', (req, res) => {
      res.json({ message: 'Lotto Game API Server' });
    });

    // Start server
    app.listen(PORT, () => {
      console.log(`Server in esecuzione sulla porta ${PORT}`);
    });

    // Gestione della chiusura pulita
    let isShuttingDown = false;
    
    async function gracefulShutdown() {
      if (isShuttingDown) return;
      isShuttingDown = true;
      
      console.log('Chiusura del server in corso...');
      
      // Chiudiamo il db usando la funzione già importata
      await closeDb();
      
      // Uscita pulita
      console.log('Server chiuso con successo');
      process.exit(0);
    }
    
    // Registriamo i gestori per diversi segnali di terminazione
    process.on('SIGINT', gracefulShutdown);
    process.on('SIGTERM', gracefulShutdown);
    process.on('uncaughtException', (err) => {
      console.error('Eccezione non gestita:', err);
      gracefulShutdown();
    });
  })
  .catch((err) => {
    console.error('Errore durante inizializzazione DB:', err);
    process.exit(1);
  });