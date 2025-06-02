import crypto from 'crypto';

export const getUser = async (db, username, password) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM users WHERE username = ?';
    
    db.get(sql, [username], (err, row) => {
      if (err) { 
        reject(err); 
      }
      else if (row === undefined) { 
        resolve(false); 
      }
      else {
        // Prepariamo l'oggetto utente pulito (senza dati sensibili)
        const user = {
          id: row.id, 
          username: row.username, 
          created_at: row.created_at
        };
        
        // Verifica password con crypto.scrypt (più sicuro di pbkdf2)
        crypto.scrypt(password, row.salt, 32, function(err, hashedPassword) {
          if (err) reject(err);
          
          // Confronto timing-safe per evitare timing attacks
          if (!crypto.timingSafeEqual(Buffer.from(row.password_hash, 'hex'), hashedPassword)) {
            resolve(false);
          } else {
            resolve(user);
          }
        });
      }
    });
  });
};

export const getUserById = async (db, userId) => {
  const query = `
    SELECT id, email, created_at
    FROM users
    WHERE id = ?
  `;
  
  return await db.get(query, [userId]);
};
