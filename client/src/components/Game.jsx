import { useState, useEffect, memo } from 'react';
import { useNavigate } from 'react-router';
import { Card, Row, Col, Badge } from 'react-bootstrap';
import API from '../API/API.mjs';

function Game({ loggedIn }) {
  const navigate = useNavigate();

  const goToSummary = () => {
    navigate('/summary', { 
      state: { 
        gameData: currentGame, 
        playerCards: playerCards 
      } 
    });
  };

  // Stati del gioco
  const [gameState, setGameState] = useState('playing');
  const [currentGame, setCurrentGame] = useState(null);
  const [currentRound, setCurrentRound] = useState(1);
  const [timer, setTimer] = useState(30);
  const [targetCard, setTargetCard] = useState(null);
  const [playerCards, setPlayerCards] = useState([]);
  const [selectedPosition, setSelectedPosition] = useState(null);
  
  // Stati del popup risultato
  const [showResultRound, setshowResultRound] = useState(false);
  const [lastRoundGuessResult, setlastRoundGuessResult] = useState(null);

  // Auto-start del gioco quando il componente si monta
  useEffect(() => {
    initializeGame();
  }, []);

  // Timer countdown
  useEffect(() => {
    let interval = null;
    if (gameState === 'playing' && timer > 0) {
      interval = setInterval(() => {
        setTimer(timer => timer - 1);
      }, 1000);
    } else if (timer === 0 && gameState === 'playing') {
      handleTimeUp();
    }
    return () => clearInterval(interval);
  }, [gameState, timer]);

  const initializeGame = async () => {
    try {      
      // Crea una nuova partita
      const gameData = await API.createGame();
      console.log('> Game data received:', gameData); 
      setCurrentGame(gameData);
      setPlayerCards(gameData.cards);
      
      // Inizia il primo round
      await startNewRound(gameData.gameId);
      
    } catch (error) {
      console.error('Errore nell\'inizializzazione del gioco:', error);
    }
  };

  const startNewRound = async (gameId = currentGame?.gameId) => {
    try {      
      // Ottieni carta per il nuovo round
      const roundData = await API.startNewRound(gameId);
      console.log('> roundData data received:', roundData);

      setTargetCard(roundData.card);
      setCurrentRound(roundData.roundNumber);
      setTimer(roundData.timeout);
      setSelectedPosition(null);
      setGameState('playing');
      
    } catch (error) {
      console.error('Errore nel caricamento del round:', error);
      if (error.message.includes('Demo game allows only one round')) {
        setGameState('game_over');
        setlastRoundGuessResult({
          isGameOver: true,
          message: 'Demo terminata! Registrati per giocare partite complete.',
          finalGame: currentGame
        });
        setshowResultRound(true);
      }
    }
  };

  const handlePositionSelect = (position) => {
    if (gameState !== 'playing') return;
    setSelectedPosition(position);
  };

  const handleTimeUp = async () => {
    const position = selectedPosition !== null ? selectedPosition : -1;
    await submitGuess(position);
  };

  const submitGuess = async (position) => {
    try {      
      setGameState('paused');

      const result = await API.submitGuess(
        currentGame.gameId,
        targetCard.id,
        position,
        currentRound
      );
      console.log('> Guess result received:', result);
      
      
      // Aggiorna lo stato del gioco
      const updatedGame = { ...result.game };
      // Assicurati che abbia sempre gameId per consistenza
      if (result.game.id && !result.game.gameId) {
        updatedGame.gameId = result.game.id;
      }
      setCurrentGame(updatedGame);
      setPlayerCards(result.game.cards);
      
      setlastRoundGuessResult({
        isCorrect: result.correct,
        correctPosition: result.correctPosition,
        selectedPosition: position,
        timeExpired: result.timeExpired,
        message: result.message,
        isGameOver: result.game.status !== 'in_progress',
        finalGame: result.game
      });
      
      setshowResultRound(true);
      
    } catch (error) {
      console.error('Errore nel submit della guess:', error);
      setGameState('playing');
    }
  };

  const handleNextAction = () => {
    setshowResultRound(false);
    
    if (lastRoundGuessResult.isGameOver) {
      setGameState('game_over');
    } else {
      startNewRound(currentGame?.gameId);
    }
  };

  const handleRestartGame = () => {
    setshowResultRound(false);
    setGameState('playing');
    setCurrentGame(null);
    setPlayerCards([]);
    setTargetCard(null);
    setCurrentRound(1);
    initializeGame();
  };

  const formatTime = (seconds) => {
    return `${seconds.toString().padStart(2, '0')}`;
  };

return (
  <div className="d-flex flex-column" style={{ minHeight: '100%', maxHeight: '100%' }}>
    <div className="d-flex justify-content-center py-4">
      <div style={{ width: '100%', maxWidth: '450px', height: 'auto' }}>
        <RenderTargetCardSection 
          targetCard={targetCard}
          currentRound={currentRound}
          timer={timer}
          formatTime={formatTime}
        />
      </div>
    </div>
    <div className="container-fluid px-5 py-4 pb-4 flex-grow-1 overflow-hidden">
      <div style={{ width: '100%', height: '100%', maxHeight: '400px' }}>
        <RenderPlayerCardsSection 
          currentGame={currentGame}
          playerCards={playerCards}
          selectedPosition={selectedPosition}
          gameState={gameState}
          handlePositionSelect={handlePositionSelect}
          submitGuess={submitGuess}
        />
      </div>
    </div>

    <RenderResultRound 
      showResultRound={showResultRound}
      lastRoundGuessResult={lastRoundGuessResult}
      loggedIn={loggedIn}
      currentGame={currentGame}
      goToSummary={goToSummary}
      handleNextAction={handleNextAction}
    />
  </div>
);
}

// Componente memoizzato per la sezione della carta target
const RenderTargetCardSection = memo(({ targetCard, currentRound, timer, formatTime }) => {
  return (
    <Row style={{ height: '200px' }}>
      <Col xs={6} className="d-flex align-items-center justify-content-center">
        <Card 
            className="border-2 me-2" 
            style={{ 
              width: '140px', 
              height: '220px',
              flexShrink: 0
            }}
          >
          <Card.Body className="d-flex flex-column p-2">                  
            <Card.Text 
                className="small text-center mb-2"
                style={{ 
                  fontSize: '0.7rem',
                  lineHeight: '1.1',
                  overflow: 'auto',
                  display: '-webkit-box',
                  WebkitLineClamp: 3,
                  WebkitBoxOrient: 'vertical',
                  minHeight: '2.5rem'
                }}
              >
                {targetCard?.text || 'Seleziona una carta!'}
              </Card.Text>

            {targetCard?.image_path && (
              <div className="flex-grow-1 d-flex align-items-center justify-content-center">
                <img 
                  src={API.getImage(targetCard.image_path)}
                  alt="Card image"
                  className="img-fluid"
                  style={{ 
                    maxHeight: '120px',
                    maxWidth: '100%',
                    objectFit: 'contain'
                  }}
                  onError={(e) => {
                    e.target.style.display = 'none';
                  }}
                />
              </div>
            )}
          </Card.Body>
        </Card>
      </Col>
      
      <Col xs={4} className="d-flex align-items-center justify-content-center">
        <div className="text-center">
          <div className="mb-1">
            <small className="text-muted">Round {currentRound}</small>
          </div>

          <div className="d-flex align-items-center justify-content-center mb-1">
            <div className="text-dark" style={{ fontSize: '2.5rem', fontWeight: 'bold', minWidth: '80px' }}>
              {formatTime(timer)}
            </div>
            <span className="text-muted ms-2" style={{ fontSize: '2.5rem' }}>⏳</span>
          </div>
        </div>
      </Col>
    </Row>
  );
});

// Componente memoizzato per la sezione delle carte del giocatore
const RenderPlayerCardsSection = memo(({ 
  currentGame, 
  playerCards, 
  selectedPosition, 
  gameState, 
  handlePositionSelect, 
  submitGuess 
}) => {
  return (
    <Card className="h-100" style={{ maxWidth: '100%' }}>
      <Card.Body className="d-flex flex-column h-100">
        <div className="d-flex justify-content-between align-items-center mb-3">
          <h6 className="mb-0">🃏 Le tue carte - Indovina la posizione!</h6>
          <small className="text-muted">
            Errori: {currentGame?.wrong_guesses || 0}/3
          </small>
        </div>
        
        <div className="w-100 overflow-auto flex-grow-1 d-flex align-items-center justify-content-center">
          <RenderCardsGrid 
            playerCards={playerCards}
            selectedPosition={selectedPosition}
            handlePositionSelect={handlePositionSelect}
          />
        </div>
        
        <div className="text-center mt-2">
          <button 
            className="btn btn-primary mt-4"
            disabled={selectedPosition === null || gameState !== 'playing'}
            onClick={() => submitGuess(selectedPosition)}
          >
            Conferma Scelta
          </button>
        </div>  
      </Card.Body>
    </Card>
  );
});

// Componente memoizzato per la griglia delle carte
const RenderCardsGrid = memo(({ playerCards, selectedPosition, handlePositionSelect }) => {
  return (
    <div className="d-flex justify-content-center align-items-center flex-nowrap gap-2 overflow-auto">
      {playerCards.map((card, index) => (
        <div key={card.id} className="d-flex align-items-center flex-shrink-0">
          <button
            className={`btn ${selectedPosition === index ? "btn-primary" : "btn-outline-secondary"} btn-sm me-2`}
            onClick={() => handlePositionSelect(index)}
            style={{ minWidth: '40px' }}
          >
            {index + 1}
          </button>
          
          <Card 
            className="border-2 me-2" 
            style={{ 
              width: '140px', 
              height: '220px',
              flexShrink: 0
            }}
          >
            <Card.Body className="d-flex flex-column p-2">                  
              <Card.Text 
                className="small text-center mb-2"
                style={{ 
                  fontSize: '0.7rem',
                  lineHeight: '1.1',
                  overflow: 'auto',
                  display: '-webkit-box',
                  WebkitLineClamp: 3,
                  WebkitBoxOrient: 'vertical',
                  minHeight: '2.5rem'
                }}
              >
                {card?.text || 'Carta'}
              </Card.Text>

              {card?.image_path && (
                <div className="flex-grow-1 d-flex align-items-center justify-content-center mb-2">
                  <img 
                    src={API.getImage(card.image_path)}
                    alt="Card image"
                    className="img-fluid"
                    style={{ 
                      maxWidth: '100%',
                      maxHeight: '120px',
                      objectFit: 'contain'
                    }}
                    onError={(e) => {
                      e.target.style.display = 'none';
                    }}
                  />
                </div>
              )}
              
              <div className="mt-auto">
                <div className="d-flex justify-content-between align-items-center">
                  <small className="text-muted" style={{ fontSize: '0.6rem' }}>Sfortuna:</small>
                  <Badge 
                    bg={card.misfortune_index > 70 ? 'danger' : 
                        card.misfortune_index > 40 ? 'warning' : 'success'}
                    style={{ fontSize: '0.6rem' }}
                  >
                    {card.misfortune_index}
                  </Badge>
                </div>
              </div>
            </Card.Body>
          </Card>
        </div>
      ))}
      
      <button
        className={`btn ${selectedPosition === playerCards.length ? "btn-primary" : "btn-outline-secondary"} btn-sm flex-shrink-0`}
        onClick={() => handlePositionSelect(playerCards.length)}
        style={{ minWidth: '40px' }}
      >
        {playerCards.length + 1}
      </button>
    </div>
  );
});

// Componente memoizzato per il modal dei risultati
const RenderResultRound = memo(({ 
  showResultRound, 
  lastRoundGuessResult, 
  loggedIn, 
  currentGame, 
  goToSummary, 
  handleNextAction 
}) => {
  if (!showResultRound) {
    return null;
  }

  return (
    <>
      <div className="modal-backdrop show"></div>
      <div className="modal show d-block" tabIndex="-1">
        <div className="modal-dialog modal-dialog-centered">
          <div className="modal-content">
            <div className="modal-body text-center p-4">
              <RenderResultContent lastRoundGuessResult={lastRoundGuessResult} />
              <RenderResultActions 
                loggedIn={loggedIn}
                currentGame={currentGame}
                lastRoundGuessResult={lastRoundGuessResult}
                goToSummary={goToSummary}
                handleNextAction={handleNextAction}
              />
            </div>
          </div>
        </div>
      </div>
    </>
  );
});

// Componente memoizzato per il contenuto del risultato
const RenderResultContent = memo(({ lastRoundGuessResult }) => {
  if (lastRoundGuessResult?.isCorrect) {
    return (
      <>
        <div style={{ fontSize: '4rem' }} className="mb-3">🎉</div>
        <h4 className="text-success mb-3">Indovinato!</h4>
      </>
    );
  }

  return (
    <>
      <h4 className="text-danger mb-3">Sbagliato!</h4>
      <p className="text-muted">
        La posizione corretta era: {(lastRoundGuessResult?.correctPosition || 0) + 1}
      </p>
    </>
  );
});

// Componente memoizzato per le azioni del risultato
const RenderResultActions = memo(({ 
  loggedIn, 
  currentGame, 
  lastRoundGuessResult, 
  goToSummary, 
  handleNextAction 
}) => {
  if (!loggedIn) {
    return (
      <div className="mt-4">
        <p className="text-muted mb-3">
          Iscriviti per giocare partite complete e salvare i tuoi progressi!
        </p>
        <div className="d-grid gap-2">
          <button className="btn btn-primary" onClick={goToSummary}>
            📊 Vai al Riepilogo
          </button>
        </div>
      </div>
    );
  }

  if (currentGame?.status === 'won' || lastRoundGuessResult?.isGameOver) {
    return (
      <div className="mt-4">
        <div className="d-grid gap-2">
          <button className="btn btn-primary" onClick={goToSummary}>
            📊 Vai al Riepilogo
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="mt-4">
      <button className="btn btn-primary" onClick={handleNextAction}>
        🚀 Prossimo Round
      </button>
    </div>
  );
});

export default Game;