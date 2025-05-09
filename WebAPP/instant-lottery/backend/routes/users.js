// File: routes/users.js
import express from 'express';
import { registerUser, loginUser, getUser, updateUserPoints } from '../db/users-dao.js';

const router = express.Router();

// POST /api/users/register - Register a new user
router.post('/register', async (req, res) => {
  const { username, password } = req.body;

  if (!username || !password) {
    return res.status(400).json({ error: 'Username and password are required' });
  }

  try {
    const newUser = await registerUser(username, password);
    res.status(201).json(newUser);
  } catch (err) {
    console.error(err);
    res.status(400).json({ error: err.message });
  }
});

// POST /api/users/login - Authenticate user
router.post('/login', async (req, res) => {
  const { username, password } = req.body;

  if (!username || !password) {
    return res.status(400).json({ error: 'Username and password are required' });
  }

  try {
    const user = await loginUser(username, password);
    res.json(user);
  } catch (err) {
    console.error(err);
    res.status(401).json({ error: err.message });
  }
});

// GET /api/users/:id - Get user by ID
router.get('/:id', async (req, res) => {
  try {
    const user = await getUser(req.params.id);
    res.json(user);
  } catch (err) {
    console.error(err);
    res.status(404).json({ error: err.message });
  }
});

// PUT /api/users/:id/points - Update user points
router.put('/:id/points', async (req, res) => {
  const { points } = req.body;

  if (typeof points !== 'number') {
    return res.status(400).json({ error: 'Points must be a number' });
  }

  try {
    await updateUserPoints(req.params.id, points);
    res.status(204).send();
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Error updating user points' });
  }
});

export default router;
