// File: routes/tasks.js - Gestisce le operazioni sui task
import express from 'express';
import { dbPromise } from '../db/db.js';
import dayjs from 'dayjs';

const router = express.Router();

// GET /api/tasks - Ottieni tutti i task
router.get('/', async (req, res) => {
  try {
    const db = await dbPromise;
    const tasks = await db.all('SELECT * FROM tasks ORDER BY due_date, priority DESC');
    res.json(tasks.map(row => mapDbTaskToTask(row)));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nel recupero dei task' });
  }
});

// GET /api/tasks/:id - Ottieni un task specifico
router.get('/:id', async (req, res) => {
  try {
    const db = await dbPromise;
    const task = await db.get('SELECT * FROM tasks WHERE id = ?', req.params.id);
    
    if (!task) {
      return res.status(404).json({ error: 'Task non trovato' });
    }
    
    res.json(mapDbTaskToTask(task));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nel recupero del task' });
  }
});

// POST /api/tasks - Crea un nuovo task
router.post('/', async (req, res) => {
  const { text, user_id, due_date, priority, completed } = req.body;
  
  if (!text) {
    return res.status(400).json({ error: 'Il testo del task è obbligatorio' });
  }
  
  try {
    const db = await dbPromise;
    const result = await db.run(
      'INSERT INTO tasks (text, user_id, due_date, priority, created_at, completed) VALUES (?, ?, ?, ?, ?, ?)',
      [
        text,
        user_id || null,
        due_date || null,
        priority || 1,
        dayjs().add(1, 'day').format('YYYY-MM-DD'),
        completed ? 1 : 0
      ]
    );
    
    const newTask = await db.get('SELECT * FROM tasks WHERE id = ?', result.lastID);
    res.status(201).json(mapDbTaskToTask(newTask));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nella creazione del task' });
  }
});

// PUT /api/tasks/:id - Aggiorna un task esistente
router.put('/:id', async (req, res) => {
  const { text, due_date, priority, completed } = req.body;
  const taskId = req.params.id;
  
  try {
    const db = await dbPromise;
    const task = await db.get('SELECT * FROM tasks WHERE id = ?', taskId);
    
    if (!task) {
      return res.status(404).json({ error: 'Task non trovato' });
    }
    
    await db.run(
      'UPDATE tasks SET text = ?, due_date = ?, priority = ?, completed = ? WHERE id = ?',
      [
        text || task.text,
        due_date || task.due_date,
        priority || task.priority,
        completed !== undefined ? (completed ? 1 : 0) : task.completed,
        taskId
      ]
    );
    
    const updatedTask = await db.get('SELECT * FROM tasks WHERE id = ?', taskId);
    res.json(mapDbTaskToTask(updatedTask));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nell\'aggiornamento del task' });
  }
});

// PATCH /api/tasks/:id/toggle - Inverti lo stato di completamento
router.patch('/:id/toggle', async (req, res) => {
  const taskId = req.params.id;
  
  try {
    const db = await dbPromise;
    const task = await db.get('SELECT * FROM tasks WHERE id = ?', taskId);
    
    if (!task) {
      return res.status(404).json({ error: 'Task non trovato' });
    }
    
    const newStatus = task.completed === 0 ? 1 : 0;
    
    await db.run('UPDATE tasks SET completed = ? WHERE id = ?', [newStatus, taskId]);
    
    const updatedTask = await db.get('SELECT * FROM tasks WHERE id = ?', taskId);
    res.json(mapDbTaskToTask(updatedTask));
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nell\'aggiornamento del task' });
  }
});

// DELETE /api/tasks/:id - Elimina un task
router.delete('/:id', async (req, res) => {
  const taskId = req.params.id;
  
  try {
    const db = await dbPromise;
    const task = await db.get('SELECT * FROM tasks WHERE id = ?', taskId);
    
    if (!task) {
      return res.status(404).json({ error: 'Task non trovato' });
    }
    
    await db.run('DELETE FROM tasks WHERE id = ?', taskId);
    
    res.status(204).send();
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Errore nell\'eliminazione del task' });
  }
});

// Helper function per convertire i campi del DB in un formato più adatto per l'API
function mapDbTaskToTask(dbTask) {
  return {
    id: dbTask.id,
    text: dbTask.text,
    completed: dbTask.completed === 1,
    userId: dbTask.user_id,
    dueDate: dbTask.due_date,
    priority: dbTask.priority,
    createdAt: dbTask.created_at
  };
}

export default router;