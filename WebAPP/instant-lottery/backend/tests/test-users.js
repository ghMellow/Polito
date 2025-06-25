import fetch from 'node-fetch';

const API_URL = 'http://localhost:3001/api/v1/users';
let testUser = null;

const formatJSON = (obj) => JSON.stringify(obj, null, 2);

// Register a new user
async function registerUser() {
  console.log('\n--- TEST: Registra un nuovo utente ---');

  const newUser = {
    username: `user_${Date.now()}`,
    password: 'test1234'
  };

  try {
    const res = await fetch(`${API_URL}/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(newUser)
    });

    const result = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Utente registrato:', formatJSON(result));

    testUser = { ...newUser, id: result.id, points: result.points };
  } catch (err) {
    console.error('Errore:', err.message);
  }
}

// Login user
async function loginUser() {
  console.log('\n--- TEST: Login utente ---');

  try {
    const res = await fetch(`${API_URL}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: testUser.username, password: testUser.password })
    });

    const result = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Utente loggato:', formatJSON(result));
  } catch (err) {
    console.error('Errore:', err.message);
  }
}

// Get user by ID
async function getUser() {
  console.log('\n--- TEST: Ottieni utente per ID ---');

  try {
    const res = await fetch(`${API_URL}/${testUser.id}`);
    const user = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Dati utente:', formatJSON(user));
  } catch (err) {
    console.error('Errore:', err.message);
  }
}

// Update user points
async function updateUserPoints() {
  console.log('\n--- TEST: Aggiorna punti utente ---');

  try {
    const res = await fetch(`${API_URL}/${testUser.id}/points`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ points: 150 })
    });

    console.log(`Status: ${res.status}`);
    console.log(res.status === 204 ? 'Aggiornamento completato' : 'Errore aggiornamento');
  } catch (err) {
    console.error('Errore:', err.message);
  }
}

// Run all user tests
async function runUserTests() {
  console.log('\n=== TEST API UTENTE ===');
  await registerUser();
  await loginUser();
  await getUser();
  await updateUserPoints();
  await getUser(); // check updated points
  console.log('\n=== FINE TEST UTENTE ===');
}

runUserTests().catch(console.error);
