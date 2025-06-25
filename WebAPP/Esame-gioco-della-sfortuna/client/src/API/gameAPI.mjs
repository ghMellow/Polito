const SERVER_URL = "http://localhost:3001";

const getImage = (imagePath) => {
  return `${SERVER_URL}/api/cards/image/${imagePath}`;
};

const createGame = async () => {
  const response = await fetch(SERVER_URL + '/api/games/new', {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    }
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.error || `HTTP error! status: ${response.status}`);
  }

  return await response.json();
};

const startNewRound = async (gameId) => {
  const response = await fetch(SERVER_URL + `/api/games/${gameId}/round`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    }
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.error || `HTTP error! status: ${response.status}`);
  }

  return await response.json();
};

const submitGuess = async (gameId, cardId, position, roundNumber) => {
  const response = await fetch(SERVER_URL + `/api/games/${gameId}/guess`, {
    method: 'POST',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      cardId: cardId,
      position: position,
      roundNumber: roundNumber
    })
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.error || `HTTP error! status: ${response.status}`);
  }

  return await response.json();
};

const GameAPI = { getImage, createGame, startNewRound, submitGuess };
export default GameAPI;