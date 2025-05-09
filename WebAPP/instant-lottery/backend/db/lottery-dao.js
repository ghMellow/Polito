/** CREAZIONE TABELLE **/

// Inizializzazione tabelle lotteria
export const initLottoTables = async (db) => {
    // Creiamo la tabella delle estrazioni se non esiste
    await db.exec(`
      CREATE TABLE IF NOT EXISTS estrazioni (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        timestamp TEXT NOT NULL,
        numero1 INTEGER NOT NULL,
        numero2 INTEGER NOT NULL,
        numero3 INTEGER NOT NULL,
        numero4 INTEGER NOT NULL,
        numero5 INTEGER NOT NULL
      )
    `);

    // Creiamo la tabella delle puntate se non esiste
    await db.exec(`
      CREATE TABLE IF NOT EXISTS puntate (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        estrazione_id INTEGER NOT NULL,
        numero1 INTEGER NOT NULL,
        numero2 INTEGER,
        numero3 INTEGER,
        costo INTEGER NOT NULL,
        numeri_indovinati INTEGER DEFAULT 0,
        vincita INTEGER DEFAULT 0,
        created_at TEXT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (estrazione_id) REFERENCES estrazioni(id) ON DELETE CASCADE
      )
    `);
};

/** OPERAZIONI CRUD **/

/** HELPER FUNCTIONS PER IL MAPPING DEI DATI **/

// Helper function per mappare una riga del database a un oggetto Estrazione
const mapRowToEstrazione = (row) => {
    return {
        id: row.id,
        timestamp: row.timestamp,
        numeri: [
            row.numero1,
            row.numero2,
            row.numero3,
            row.numero4,
            row.numero5
        ]
    };
};

// Helper function per mappare una riga del database a un oggetto Puntata
const mapRowToPuntata = (row) => {
    return {
        id: row.id,
        userId: row.user_id,
        estrazioneId: row.estrazione_id,
        numeriPuntati: [row.numero1, row.numero2, row.numero3].filter(n => n !== null),
        costo: row.costo,
        numeriIndovinati: row.numeri_indovinati,
        vincita: row.vincita,
        createdAt: row.created_at
    };
};

// Helper function per mappare una riga del database con join estrazione a un oggetto PuntataCompleta
const mapRowToPuntataCompleta = (row) => {
    return {
        id: row.id,
        estrazioneId: row.estrazione_id,
        numeriPuntati: [row.numero1, row.numero2, row.numero3].filter(n => n !== null),
        numeriEstratti: [row.e_numero1, row.e_numero2, row.e_numero3, row.e_numero4, row.e_numero5],
        costo: row.costo,
        numeriIndovinati: row.numeri_indovinati,
        vincita: row.vincita,
        timestamp: row.timestamp,
        createdAt: row.created_at
    };
};

/** ESTRAZIONI OPERATIONS **/

// Crea una nuova estrazione
export const createEstrazione = async (db) => {
    // Genera 5 numeri casuali unici tra 1 e 90
    const numeri = generateUniqueRandomNumbers(5, 1, 90);

    const result = await db.run(
        'INSERT INTO estrazioni(timestamp, numero1, numero2, numero3, numero4, numero5) VALUES (?, ?, ?, ?, ?, ?)',
        [new Date().toISOString(), ...numeri]
    );

    return {
        id: result.lastID,
        timestamp: new Date().toISOString(),
        numeri
    };
};

// Ottieni l'ultima estrazione
export const getLatestEstrazione = async (db) => {
    const estrazione = await db.get(
        'SELECT id, timestamp, numero1, numero2, numero3, numero4, numero5 FROM estrazioni ORDER BY id DESC LIMIT 1'
    );

    if (!estrazione) {
        return null;
    }

    return mapRowToEstrazione(estrazione);
};

/** PUNTATE OPERATIONS **/

// Crea una nuova puntata
export const createPuntata = async (db, userId, estrazioneId, numeri, getUser, updateUserPoints) => {
    // Verifica che l'utente abbia abbastanza punti
    const user = await getUser(db, userId);
    const costo = numeri.length * 5;

    if (user.points < costo) {
        throw new Error('Punti insufficienti per effettuare la puntata');
    }

    // Verifica che non ci siano già puntate per questa estrazione
    const esistentePuntata = await db.get(
        'SELECT id FROM puntate WHERE user_id = ? AND estrazione_id = ?',
        [userId, estrazioneId]
    );

    if (esistentePuntata) {
        throw new Error('Hai già effettuato una puntata per questa estrazione');
    }

    // Prepara i dati per l'inserimento
    const numero1 = numeri[0];
    const numero2 = numeri.length > 1 ? numeri[1] : null;
    const numero3 = numeri.length > 2 ? numeri[2] : null;

    // Inserisci la puntata
    const result = await db.run(
        'INSERT INTO puntate(user_id, estrazione_id, numero1, numero2, numero3, costo, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)',
        [userId, estrazioneId, numero1, numero2, numero3, costo, new Date().toISOString()]
    );

    // Aggiorna i punti dell'utente
    await updateUserPoints(db, userId, user.points - costo);

    return {
        id: result.lastID,
        userId,
        estrazioneId,
        numeri,
        costo
    };
};

// Verifica le vincite per un'estrazione
export const verificaVincite = async (db, estrazioneId, getUser, updateUserPoints) => {
    // Ottieni l'estrazione
    const estrazione = await db.get(
        'SELECT numero1, numero2, numero3, numero4, numero5 FROM estrazioni WHERE id = ?',
        [estrazioneId]
    );

    if (!estrazione) {
        throw new Error('Estrazione non trovata');
    }

    const numeriEstratti = [
        estrazione.numero1,
        estrazione.numero2,
        estrazione.numero3,
        estrazione.numero4,
        estrazione.numero5
    ];

    // Ottieni tutte le puntate per questa estrazione
    const puntate = await db.all(
        'SELECT id, user_id, numero1, numero2, numero3, costo FROM puntate WHERE estrazione_id = ?',
        [estrazioneId]
    );

    // Calcola le vincite per ogni puntata
    for (const puntata of puntate) {
        const numeriPuntata = [puntata.numero1];
        if (puntata.numero2) numeriPuntata.push(puntata.numero2);
        if (puntata.numero3) numeriPuntata.push(puntata.numero3);

        // Conta quanti numeri sono stati indovinati
        const numeriIndovinati = numeriPuntata.filter(n => numeriEstratti.includes(n)).length;

        // Calcola la vincita in base ai numeri indovinati
        let vincita = calcolaVincita(numeriPuntata.length, numeriIndovinati);

        // Aggiorna la puntata con i numeri indovinati e la vincita
        await db.run(
            'UPDATE puntate SET numeri_indovinati = ?, vincita = ? WHERE id = ?',
            [numeriIndovinati, vincita, puntata.id]
        );

        // Se c'è una vincita, aggiorna i punti dell'utente
        if (vincita > 0) {
            const user = await getUser(db, puntata.user_id);
            await updateUserPoints(db, puntata.user_id, user.points + vincita);
        }
    }

    return puntate.length;
};

// Helper function per calcolare la vincita
const calcolaVincita = (numeriPuntati, numeriIndovinati) => {
    if (numeriPuntati === 1) {
        // Se ha puntato su 1 numero e lo ha indovinato, vince 15 punti
        return numeriIndovinati === 1 ? 15 : 0;
    } else if (numeriPuntati === 2) {
        // Se ha puntato su 2 numeri:
        if (numeriIndovinati === 1) return 5;      // 1 numero indovinato: 5 punti
        else if (numeriIndovinati === 2) return 50; // 2 numeri indovinati: 50 punti
        return 0;
    } else if (numeriPuntati === 3) {
        // Se ha puntato su 3 numeri:
        if (numeriIndovinati === 1) return 0;       // 1 numero indovinato: 0 punti
        else if (numeriIndovinati === 2) return 25;  // 2 numeri indovinati: 25 punti
        else if (numeriIndovinati === 3) return 200; // 3 numeri indovinati: 200 punti
        return 0;
    }
    return 0;
};

// Ottieni le puntate di un utente
export const getPuntateUtente = async (db, userId) => {
    const puntate = await db.all(`
      SELECT p.id, p.estrazione_id, p.numero1, p.numero2, p.numero3, 
             p.costo, p.numeri_indovinati, p.vincita, p.created_at,
             e.numero1 as e_numero1, e.numero2 as e_numero2, e.numero3 as e_numero3, 
             e.numero4 as e_numero4, e.numero5 as e_numero5, e.timestamp
      FROM puntate p
      JOIN estrazioni e ON p.estrazione_id = e.id
      WHERE p.user_id = ?
      ORDER BY p.created_at DESC
      LIMIT 20
    `, [userId]);

    return puntate.map(mapRowToPuntataCompleta);
};

// Utility function per generare numeri casuali unici
const generateUniqueRandomNumbers = (count, min, max) => {
    const numbers = new Set();
    while (numbers.size < count) {
        const num = Math.floor(Math.random() * (max - min + 1)) + min;
        numbers.add(num);
    }
    return [...numbers];
};