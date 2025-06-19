import crypto from 'crypto';

const BLACKLISTED_EMAIL_PATTERNS = [
  /admin@admin\.com$/i,
  /test@test\.com$/i,
  /demo@demo\.com$/i,
  /root@.*$/i,
  /administrator@.*$/i,
];

const isBlacklistedEmail = (email) => {
  return BLACKLISTED_EMAIL_PATTERNS.some(pattern => pattern.test(email));
};

export const getUser = async (db, email, password) => {
  return new Promise((resolve, reject) => {
    if (isBlacklistedEmail(email)) {
      resolve(false);
      return;
    }

    const sql = 'SELECT * FROM users WHERE email = ?';

    db.get(sql, [email], (err, row) => {
      if (err) {
        reject(err);
      }
      else if (row === undefined) {
        resolve(false);
      }
      else {
        const user = {
          id: row.id,
          username: row.username,
          email: row.email
        };

        crypto.scrypt(password, row.salt, 32, function (err, hashedPassword) {
          if (err) reject(err);

          // Confronto timing-safe per evitare timing attacks
          if (!crypto.timingSafeEqual(Buffer.from(row.password, 'hex'), hashedPassword)) {
            resolve(false);
          } else {
            resolve(user);
          }
        });
      }
    });
  });
};
