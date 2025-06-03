import crypto from 'crypto';

export const getUser = async (db, email, password) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM users WHERE email = ?';
    
    db.get(sql, [email], (err, row) => {
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
          email: row.email, 
          image_path: row.image_path || "guest.png",
        };
        
        // Verifica password con crypto.scrypt (più sicuro di pbkdf2)
        crypto.scrypt(password, row.salt, 32, function(err, hashedPassword) {
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
