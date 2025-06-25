import { dbPromise } from './db.js';

/** CREAZIONE TABELLE **/

// Inizializzazione tabelle users
export const initUserTable = async (db) => {
    // Creiamo la tabella degli utenti se non esiste
    await db.exec(`
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            points INTEGER DEFAULT 100,
            created_at TEXT NOT NULL
        )
    `);
};


/** OPERAZIONI CRUD **/


/** USERS OPERATIONS **/

// Registra un nuovo utente
export const registerUser = async (username, password) => {
    const db = await dbPromise;
    try {
        const result = await db.run(
            'INSERT INTO users(username, password, points, created_at) VALUES (?, ?, 100, ?)',
            [username, password, new Date().toISOString()]
        );
        return { id: result.lastID, username, points: 100 };
    } catch (err) {
        if (err.message.includes('UNIQUE constraint failed')) {
            throw new Error('Username già esistente');
        }
        throw err;
    }
};

// Login utente
export const loginUser = async (username, password) => {
    const db = await dbPromise;
    const user = await db.get(
        'SELECT id, username, points FROM users WHERE username = ? AND password = ?',
        [username, password]
    );

    if (!user) {
        throw new Error('Credenziali non valide');
    }

    return user;
};

// Ottieni dati utente
export const getUser = async (userId) => {
    const db = await dbPromise;
    const user = await db.get(
        'SELECT id, username, points FROM users WHERE id = ?',
        [userId]
    );

    if (!user) {
        throw new Error('Utente non trovato');
    }

    return user;
};

// Aggiorna punti utente
export const updateUserPoints = async (userId, points) => {
    const db = await dbPromise;
    await db.run(
        'UPDATE users SET points = ? WHERE id = ?',
        [points, userId]
    );
};