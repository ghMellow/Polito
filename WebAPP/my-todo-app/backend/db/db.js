/* Data Access Object (DAO) module for accessing Todo items */
/* Functional programming approach */

import sqlite3 from 'sqlite3';
import { open } from 'sqlite';

// Esportiamo la promessa del database per permetterne l'utilizzo in altri moduli
const dbPromise = open({
  filename: './db/database.sqlite',
  driver: sqlite3.Database
}).then(async db => {
  // Abilitiamo le foreign keys
  await db.exec('PRAGMA foreign_keys = ON');
  
  // Creiamo la tabella dei task se non esiste
  await db.exec(`
    CREATE TABLE IF NOT EXISTS tasks (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      text TEXT NOT NULL,
      completed INTEGER DEFAULT 0,
      user_id INTEGER,
      due_date TEXT,
      priority INTEGER DEFAULT 1,
      created_at TEXT NOT NULL
    )
  `);
  
  return db;
});

// Export named, non default
export { dbPromise };

/** TODOS OPERATIONS **/

// get all todos
export const listTodos = () => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM todo ORDER BY dueDate, priority DESC';
    db.all(sql, [], (err, rows) => {
      if (err) {
        reject(err);
      } else {
        const todos = rows.map(row => mapRowToTodo(row));
        resolve(todos);
      }
    });
  });
};

// get todos by completion status
export const listTodosByStatus = (completed) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM todo WHERE completed = ? ORDER BY dueDate, priority DESC';
    db.all(sql, [completed ? 1 : 0], (err, rows) => {
      if (err) {
        reject(err);
      } else {
        const todos = rows.map(row => mapRowToTodo(row));
        resolve(todos);
      }
    });
  });
};

// get a todo given its id
export const getTodo = (id) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM todo WHERE id = ?';
    db.get(sql, [id], (err, row) => {
      if (err) {
        reject(err);
      } else if (row === undefined) {
        resolve({ error: "Todo not found, check the inserted id." });
      } else {
        resolve(mapRowToTodo(row));
      }
    });
  });
};

// add a new todo
export const addTodo = (todo) => {
  return new Promise((resolve, reject) => {
    const sql = 'INSERT INTO todo(text, userId, dueDate, priority, createdAt, completed) VALUES (?, ?, ?, ?, ?, ?)';
    db.run(sql, [
      todo.text, 
      todo.userId, 
      todo.dueDate, 
      todo.priority, 
      todo.createdAt || new Date().toISOString(),
      todo.completed ? 1 : 0
    ], function(err) {
      if (err)
        reject(err);
      else 
        resolve(this.lastID);
    });
  });
};

// update an existing todo
export const updateTodo = (todo) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE todo SET text = ?, dueDate = ?, priority = ?, completed = ? WHERE id = ?';
    db.run(sql, [
      todo.text, 
      todo.dueDate, 
      todo.priority, 
      todo.completed ? 1 : 0, 
      todo.id
    ], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// mark a todo as completed or not completed
export const toggleTodoCompletion = (todoId, completed) => {
  return new Promise((resolve, reject) => {
    const sql = 'UPDATE todo SET completed = ? WHERE id = ?';
    db.run(sql, [completed ? 1 : 0, todoId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// delete a todo
export const deleteTodo = (todoId) => {
  return new Promise((resolve, reject) => {
    const sql = 'DELETE FROM todo WHERE id = ?';
    db.run(sql, [todoId], function(err) {
      if (err)
        reject(err);
      else
        resolve(this.changes);
    });
  });
};

// get todos due today
export const getTodosForToday = () => {
  return new Promise((resolve, reject) => {
    const today = new Date().toISOString().split('T')[0];
    const sql = "SELECT * FROM todo WHERE date(dueDate) = date(?) AND completed = 0 ORDER BY priority DESC";
    db.all(sql, [today], (err, rows) => {
      if (err) {
        reject(err);
      } else {
        const todos = rows.map(row => mapRowToTodo(row));
        resolve(todos);
      }
    });
  });
};

// Helper function to map a database row to a Todo object
const mapRowToTodo = (row) => {
  return {
    id: row.id,
    text: row.text,
    completed: row.completed === 1,
    userId: row.userId,
    dueDate: row.dueDate,
    priority: row.priority,
    createdAt: row.createdAt
  };
};

// Close the database connection (useful for graceful shutdown)
export const closeDb = () => {
  return new Promise((resolve, reject) => {
    db.close(err => {
      if (err) reject(err);
      else resolve();
    });
  });
};