// create a new game
export const createGame = (db, userId = null) => {
  return new Promise((resolve, reject) => {
    const sql = 'INSERT INTO games (user_id, status, total_cards, wrong_guesses) VALUES (?, ?, ?, ?)';
    db.run(sql, [userId, 'in_progress', 3, 0], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.lastID);
    });
  });
};

// get a game by ID
export const getGame = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM games WHERE id = ?';
    db.get(sql, [gameId], (err, row) => {
      if (err) {
        reject(err);
      } else if (row === undefined) {
        resolve({ error: "Game not found." });
      } else {
        resolve(row);
      }
    });
  });
};

export const getUserGamesInProgress = (db, userId) => {
  return new Promise((resolve, reject) => {
    const sql = `
      SELECT * FROM games 
      WHERE user_id = ? AND status == 'in_progress' 
      ORDER BY completed_at DESC
    `;
    db.all(sql, [userId], (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows);
    });
  });
};

// update game status (won/lost)
export const updateGameStatus = (db, gameId, status, totalCards) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET status = ?, total_cards = ?, completed_at = CURRENT_TIMESTAMP WHERE id = ?';
    db.run(sql, [status, totalCards, gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// increment wrong guesses
export const incrementWrongGuesses = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET wrong_guesses = wrong_guesses + 1 WHERE id = ?';
    db.run(sql, [gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// increment total cards (when player wins a card)
export const incrementTotalCards = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET total_cards = total_cards + 1 WHERE id = ?';
    db.run(sql, [gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// get user's completed games (for history)
export const getUserGameHistory = (db, userId) => {
  return new Promise((resolve, reject) => {
    const sql = `
      SELECT * FROM games 
      WHERE user_id = ? AND status != 'in_progress' 
      ORDER BY completed_at DESC
    `;
    db.all(sql, [userId], (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows);
    });
  });
};

// delete a game (for cleanup)
export const deleteGame = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'DELETE FROM games WHERE id = ?';
    db.run(sql, [gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};