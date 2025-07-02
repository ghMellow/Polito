import express from 'express';
import passport from 'passport';

// Route /api/auth/
const router = express.Router();

router.post('/login', passport.authenticate('local'), function (req, res) {
  return res.status(201).json(req.user);
});

router.get('/session', (req, res) => {
  if (req.isAuthenticated()) {
    res.json(req.user);
  } else {
    res.status(401).json({ error: 'Not authenticated' });
  }
});

router.delete('/logout', (req, res) => {
  req.logout(() => {
    res.end();
  });
});

export default router;