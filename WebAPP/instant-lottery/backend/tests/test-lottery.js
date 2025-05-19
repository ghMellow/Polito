// test-lottery-api.js - Script for testing lottery API endpoints
import fetch from 'node-fetch';

const API_URL = 'http://localhost:3001/api/v1/lottery';

// Utility for formatting JSON
const formatJSON = (obj) => JSON.stringify(obj, null, 2);

// Initialize lottery DB tables
async function initLottoTables() {
  console.log('\n--- TEST: Initialize Lottery Tables ---');
  try {
    const res = await fetch(`${API_URL}/init`, { method: 'POST' });
    console.log(`Status: ${res.status}`);
    const result = await res.json();
    console.log('Response:', formatJSON(result));
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Create new lottery draw
async function createEstrazione() {
  console.log('\n--- TEST: Create New Draw ---');
  try {
    const res = await fetch(`${API_URL}/estrazioni`, { method: 'POST' });
    const draw = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Draw:', formatJSON(draw));
    return draw;
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Get latest draw
async function getLatestEstrazione() {
  console.log('\n--- TEST: Get Latest Draw ---');
  try {
    const res = await fetch(`${API_URL}/estrazioni/latest`);
    const draw = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Latest Draw:', formatJSON(draw));
    return draw;
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Create new bet
async function createPuntata(userId, estrazioneId, numeri) {
  console.log('\n--- TEST: Create Bet ---');
  try {
    const res = await fetch(`${API_URL}/puntate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ userId, estrazioneId, numeri })
    });
    const bet = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Bet:', formatJSON(bet));
    return bet;
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Verify winnings
async function verificaVincite(estrazioneId) {
  console.log('\n--- TEST: Verify Winnings ---');
  try {
    const res = await fetch(`${API_URL}/estrazioni/${estrazioneId}/verifica`, { method: 'POST' });
    const result = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('Result:', formatJSON(result));
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Get bets by user
async function getPuntateUtente(userId) {
  console.log(`\n--- TEST: Get Bets for User ${userId} ---`);
  try {
    const res = await fetch(`${API_URL}/puntate/user/${userId}`);
    const bets = await res.json();
    console.log(`Status: ${res.status}`);
    console.log('User Bets:', formatJSON(bets));
  } catch (err) {
    console.error('Error:', err.message);
  }
}

// Run all tests
async function runAllTests() {
  console.log('=== STARTING LOTTERY API TESTS ===');

  await initLottoTables();

  const draw = await createEstrazione();
  const latest = await getLatestEstrazione();

  const userId = 1; // Replace with a valid userId in your DB
  const numeri = [5, 12, 33]; // Example valid numbers

  if (latest && latest.id) {
    const bet = await createPuntata(userId, latest.id, numeri);
    await verificaVincite(latest.id);
    await getPuntateUtente(userId);
  }

  console.log('=== END OF TESTS ===');
}

runAllTests().catch(console.error);
