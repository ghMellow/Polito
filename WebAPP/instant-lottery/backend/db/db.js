// File: db/db.js
import sqlite3 from 'sqlite3';
import { open } from 'sqlite';
import { initUserTable } from './users-dao.js';
import { initLottoTables } from './lottery-dao.js';

// Ottimizzazione per debugging in ambiente di sviluppo
if (process.env.NODE_ENV === 'development') {
  sqlite3.verbose();
}

/**
 * Crea e inizializza la connessione al database
 * - Abilita foreign keys
 * - Inizializza le tabelle necessarie
 */
const dbPromise = open({
  filename: './db/lotto-database.sqlite',
  driver: sqlite3.Database
}).then(async db => {
  console.log('Connessione al database stabilita');
  
  // Abilita foreign keys
  await db.exec('PRAGMA foreign_keys = ON');
  
  // Inizializzazione delle tabelle
  console.log('Inizializzazione tabelle utenti...');
  await initUserTable(db);
  
  console.log('Inizializzazione tabelle lotteria...');
  await initLottoTables(db);
  
  console.log('Database inizializzato con successo');
  return db;
}).catch(err => {
  console.error('Errore durante l\'inizializzazione del database:', err);
  throw err; // Rilanciamo l'errore per gestirlo altrove
});

/**
 * Flag per tenere traccia dello stato della connessione
 */
let isDbClosed = false;

/**
 * Chiude la connessione al database
 * Da utilizzare per shutdown puliti dell'applicazione
 */
export const closeDb = async () => {
  try {
    // Verifichiamo che il database non sia già stato chiuso
    if (isDbClosed) {
      console.log('Database già chiuso, nessuna azione necessaria');
      return;
    }
    
    const db = await dbPromise;
    console.log('Chiusura connessione database...');
    await db.close();
    isDbClosed = true;
    console.log('Connessione database chiusa con successo');
  } catch (err) {
    // Gestiamo in modo specifico l'errore di database già chiuso
    if (err.code === 'SQLITE_MISUSE' && err.message.includes('Database handle is closed')) {
      console.log('Database già chiuso');
      isDbClosed = true;
      return;
    }
    
    console.error('Errore durante la chiusura del database:', err);
  }
};

// Esportiamo la promessa del database
export { dbPromise };