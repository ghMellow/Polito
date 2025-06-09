export const getCard = (db, id) => {
  return new Promise((resolve, reject) => {
    const sql = 'SELECT * FROM cards WHERE id = ?';
    db.get(sql, [id], (err, row) => {
      if (err) {
        reject(err);
      } else if (row === undefined) {
        resolve({ error: "Card not available, check the inserted id." });
      } else {
        resolve(row);
      }
    });
  });
};

export const getAllCards = (db, hideMisfortune = false) => {
  return new Promise((resolve, reject) => {
    let sql = '';
    if (hideMisfortune == false) {
      sql = 'SELECT * FROM cards';
    } else {
      sql = 'SELECT id, text, image_path FROM cards';
    }
    
    db.all(sql, [], (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows);
    });
  });
};