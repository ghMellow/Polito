// create a new game
export const createGame = (db, userId, created_at) => {
  return new Promise((resolve, reject) => {
    const sql = 'INSERT INTO games (user_id, status, total_cards, wrong_guesses, correct_guesses, created_at) VALUES (?, ?, ?, ?, ?, ?)';
    db.run(sql, [userId, 'in_progress', 3, 0, 0, created_at], function(err) {
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
      ORDER BY created_at DESC
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
export const updateGameStatus = (db, gameId, status) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET status = ? WHERE id = ?';
    db.run(sql, [status, gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

export const incrementWrongGuesses = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET wrong_guesses = wrong_guesses + 1, total_cards = total_cards + 1 WHERE id = ?';
    db.run(sql, [gameId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

export const incrementCorrectGuesses = (db, gameId) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE games SET correct_guesses = correct_guesses + 1, total_cards = total_cards + 1 WHERE id = ?';
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
      ORDER BY created_at DESC
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