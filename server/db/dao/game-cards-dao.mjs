// add initial cards to a game (3 random cards at game start)
export const addInitialCards = (db, gameId, cardIds) => {
  return new Promise((resolve, reject) => {
    const sql = 'INSERT INTO game_cards (game_id, card_id, round_number, won, initial_card) VALUES (?, ?, ?, ?, ?)';
    const stmt = db.prepare(sql);
    
    let completed = 0;
    let errors = [];
    
    cardIds.forEach(cardId => {
      stmt.run([gameId, cardId, null, true, true], (err) => {
        if (err) {
          errors.push(err);
        }
        completed++;
        
        if (completed === cardIds.length) {
          stmt.finalize();
          if (errors.length > 0) {
            reject(errors[0]);
          } else {
            resolve(completed);
          }
        }
      });
    });
  });
};

// add a round card result (won or lost)
export const addRoundCard = (db, gameId, cardId, roundNumber, won) => {
  return new Promise((resolve, reject) => {
    const sql = 'INSERT INTO game_cards (game_id, card_id, round_number, won, initial_card) VALUES (?, ?, ?, ?, ?)';
    db.run(sql, [gameId, cardId, roundNumber, won, false], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.lastID);
    });
  });
};

// get all cards of a game (won cards only)
export const getGameWonCards = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = `
      SELECT c.*, gc.round_number, gc.initial_card, gc.created_at as card_acquired_at
      FROM cards c
      JOIN game_cards gc ON c.id = gc.card_id
      WHERE gc.game_id = ? AND gc.won = 1
      ORDER BY c.misfortune_index ASC
    `;
    db.all(sql, [gameId], (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows);
    });
  });
};

// get all cards used in a game (for avoiding duplicates)
export const getGameUsedCards = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT card_id FROM game_cards WHERE game_id = ?';
    db.all(sql, [gameId], (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows.map(row => row.card_id));
    });
  });
};

// get game history with card details for a user
export const getUserGameHistoryWithCards = (db, userId) => {
  return new Promise((resolve, reject) => {
    const sql = `
      SELECT *
      FROM games g
      JOIN game_cards gc ON g.id = gc.game_id
      JOIN cards c ON gc.card_id = c.id
      WHERE g.user_id = ? AND g.status != 'in_progress'
      ORDER BY g.completed_at DESC, gc.round_number ASC
    `;
    db.all(sql, [userId], (err, rows) => {
      if (err)
        reject(err);
      else {
        // Raggruppa i risultati per partita
        const gamesMap = new Map();
        
        rows.forEach(row => {
          if (!gamesMap.has(row.game_id)) {
            gamesMap.set(row.game_id, {
              id: row.game_id,
              status: row.status,
              total_cards: row.total_cards,
              wrong_guesses: row.wrong_guesses,
              correct_guesses: row.correct_guesses,
              created_at: row.created_at,
              completed_at: row.completed_at,
              cards: []
            });
          }
          
          gamesMap.get(row.game_id).cards.push({
            id: row.card_id,
            text: row.text,
            image_path: row.image_path,
            misfortune_index: row.misfortune_index,
            round_number: row.round_number,
            won: row.won,
            initial_card: row.initial_card
          });
        });
        
        resolve(Array.from(gamesMap.values()));
      }
    });
  });
};

// get current round number for a game
export const getCurrentRoundNumber = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT MAX(round_number) as max_round FROM game_cards WHERE game_id = ? AND round_number IS NOT NULL';
    db.get(sql, [gameId], (err, row) => {
      if (err)
        reject(err);
      else
        resolve((row.max_round || 0) + 1);
    });
  });
};

// delete all cards of a game (for cleanup)
export const deleteGameCards = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'DELETE FROM game_cards WHERE game_id = ?';
    db.run(sql, [gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};