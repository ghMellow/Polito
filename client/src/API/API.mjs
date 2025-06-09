const SERVER_URL = "http://localhost:3001";

const logIn = async (credentials) => {
  const response = await fetch(SERVER_URL + '/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(credentials),
  });
  if (response.ok) {
    const user = await response.json();
    return user;
  }
  else if (response.status === 401) {
    throw "Email o password errate";
  } else {
    const errDetails = await response.text();
    throw errDetails;
  }
};

const getUserInfo = async () => {
  const response = await fetch(SERVER_URL + '/api/auth/session', {
    credentials: 'include',
  });
  const user = await response.json();
  if (response.ok) {
    return user;
  } else {
    throw user;  // an object with the error coming from the server
  }
};

const logOut = async () => {
  const response = await fetch(SERVER_URL + '/api/auth/logout', {
    method: 'DELETE',
    credentials: 'include'
  });
  if (response.ok)
    return null;
}

const getUserProfile = async () => {
  const response = await fetch(SERVER_URL + '/api/users/profile', {
    method: 'GET',
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    }
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
  }

  return await response.json();
};

const getImage = (imagePath) => {
  return `${SERVER_URL}/api/cards/image/${imagePath}`;
};

const createGame = async () => {
  const response = await fetch(SERVER_URL + '/api/games', {
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


const API = { logIn, getUserInfo, logOut, getUserProfile, getImage, createGame, startNewRound, submitGuess};
export default API;