import express from 'express';
import morgan from 'morgan';
import cors from 'cors';
import passport from 'passport';
import LocalStrategy from 'passport-local';
import session from 'express-session';
import { getUser } from './db/dao/users-dao.mjs';
import { dbPromise } from './db/db.mjs';

import authRoutes from './routes/auth.mjs';
import gameRoutes from './routes/games.mjs';
import userRoutes from './routes/users.mjs';
import cardRoutes from './routes/cards.mjs';


// init express
const app = express();
const port = 3001;

// middleware
app.use(express.json());
app.use(morgan('dev'));

// CORS configuration
const corsOptions = {
  origin: 'http://localhost:5173',
  optionsSuccessStatus: 200,
  credentials: true
};
app.use(cors(corsOptions));

app.use('/images', express.static('public/images'));

// Passport configuration
passport.use(new LocalStrategy(
  { usernameField: 'email', passwordField: 'password' },
  async function verify(email, password, cb) {
    const user = await getUser(dbPromise, email, password);
    if(!user)
      return cb(null, false, 'Email o password sbagliate.');
      
    return cb(null, user);
  }
));

passport.serializeUser(function (user, cb) {
  cb(null, user);
});

passport.deserializeUser(function (user, cb) {
  return cb(null, user);
});
console.log('Passport configurato con successo');

// Session configuration
app.use(session({
  secret: "shhhhh... it's a secret!",
  resave: false,
  saveUninitialized: false
}));

app.use(passport.authenticate('session'));

// Routes
app.use('/api/auth', authRoutes);
app.use('/api/games', gameRoutes);
app.use('/api/users', userRoutes);
app.use('/api/cards', cardRoutes);
app.use('/api/*', (req, res) => {
  res.status(404).json({ error: 'API endpoint not found' });
});

// activate the server
app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`);
});