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

export const getRandomCards = (db, limit, hideMisfortune = false, excludeIds = []) => {
  return new Promise((resolve, reject) => {
    let sql = '';
    if (hideMisfortune == false) {
      sql = 'SELECT * FROM cards';
    } else {
      sql = 'SELECT id, text, image_path FROM cards';
    }
    
    let params = [];
    if (excludeIds.length > 0) {
      const placeholders = excludeIds.map(() => '?').join(',');
      sql += ` WHERE id NOT IN (${placeholders})`;
      params = excludeIds;
    }
    
    sql += ' ORDER BY RANDOM() LIMIT ?';
    params.push(limit);
    
    db.all(sql, params, (err, rows) => {
      if (err)
        reject(err);
      else
        resolve(rows);
    });
  });
};