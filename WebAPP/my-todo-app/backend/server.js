// File: server.js - Server principale dell'applicazione
import express from 'express';
import cors from 'cors';
import tasksRouter from './routes/tasks.js';

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Routes
app.use('/api/tasks', tasksRouter);

// Route di base
app.get('/', (req, res) => {
  res.json({ message: 'Todo API Server' });
});

// Avvio del server
app.listen(PORT, () => {
  console.log(`Server in esecuzione sulla porta ${PORT}`);
});