import sqlite3 from 'sqlite3';

const dbPromise = new sqlite3.Database('./db/misfortune-game-database.sqlite', (err) => {
  if (err) throw err;
});

dbPromise.run('PRAGMA foreign_keys = ON');

export { dbPromise }; 