
import { useState, useEffect } from 'react';
import { Link } from 'react-router';
import { Card, Row, Col, Badge } from 'react-bootstrap';
import API from '../API/API.mjs';

function Game({ loggedIn, user }) {
  // Stati del gioco
  const [gameState, setGameState] = useState('playing');
  const [currentGame, setCurrentGame] = useState(null);
  const [currentRound, setCurrentRound] = useState(1);
  const [timer, setTimer] = useState(30);
  const [targetCard, setTargetCard] = useState(null);
  const [playerCards, setPlayerCards] = useState([]);
  const [selectedPosition, setSelectedPosition] = useState(null);
  
  // Stati del popup risultato
  const [showResultModal, setShowResultModal] = useState(false);
  const [lastGuessResult, setLastGuessResult] = useState(null);

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
        setLastGuessResult({
          isGameOver: true,
          message: 'Demo terminata! Registrati per giocare partite complete.',
          finalGame: currentGame
        });
        setShowResultModal(true);
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
      
      setLastGuessResult({
        isCorrect: result.correct,
        correctPosition: result.correctPosition,
        selectedPosition: position,
        timeExpired: result.timeExpired,
        message: result.message,
        isGameOver: result.game.status !== 'in_progress',
        finalGame: result.game
      });
      
      setShowResultModal(true);
      
    } catch (error) {
      console.error('Errore nel submit della guess:', error);
      setGameState('playing');
    }
  };

  const handleNextAction = () => {
    setShowResultModal(false);
    
    if (lastGuessResult.isGameOver) {
      setGameState('game_over');
    } else {
      startNewRound(currentGame?.gameId);
    }
  };

  const handleRestartGame = () => {
    setShowResultModal(false);
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

  // eseguita quando?
  if (gameState === 'game_over' && !showResultModal) {
    return <RenderGameOverState />;
  }

return (
  <div className="d-flex flex-column" style={{ minHeight: '100%', maxHeight: '100%' }}>
    {/* Sezione Target Card più compatta */}
    <div className="d-flex justify-content-center py-4">
      <div style={{ width: '100%', maxWidth: '450px', height: 'auto' }}>
        <RenderTargetCardSection />
      </div>
    </div>
    
    {/* Sezione Player Cards con più spazio */}
    <div className="container-fluid px-5 py-4 pb-4 flex-grow-1 overflow-hidden">
      <div style={{ width: '100%', height: '100%', maxHeight: '400px' }}>
        <RenderPlayerCardsSection />
      </div>
    </div>
    
    <RenderResultModal />
  </div>
);

function RenderTargetCardSection() {
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
}

function RenderPlayerCardsSection() {
  return (
    <Card className="h-100" style={{ maxWidth: '100%' }}>
      <Card.Body className="d-flex flex-column h-100">
        <div className="d-flex justify-content-between align-items-center mb-3">
          <h6 className="mb-0">🃏 Le tue carte - Seleziona la posizione</h6>
          <small className="text-muted">
            Errori: {currentGame?.wrongGuesses || 0}/3
          </small>
        </div>
        
        <div className="w-100 overflow-auto flex-grow-1 d-flex align-items-center justify-content-center">
          <RenderCardsGrid />
        </div>
        
        <RenderSelectedPositionInfo />
      </Card.Body>
    </Card>
  );
}

function RenderCardsGrid() {
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
}

  function RenderSelectedPositionInfo() {
    return (
      <div className="text-center mt-2">
        <button 
          className="btn btn-primary mt-4"
          disabled={selectedPosition === null || gameState !== 'playing'}
          onClick={() => submitGuess(selectedPosition)}
        >
          Conferma Scelta
        </button>
      </div>
    );
  }

  function RenderResultModal() {
    if (!showResultModal) {
      return null;
    }

    return (
      <>
        <div className="modal-backdrop show"></div>
        <div className="modal show d-block" tabIndex="-1">
          <div className="modal-dialog modal-dialog-centered">
            <div className="modal-content">
              <div className="modal-body text-center p-4">
                <RenderResultContent />
                <RenderResultActions />
              </div>
            </div>
          </div>
        </div>
      </>
    );
  }

  function RenderResultContent() {
    if (lastGuessResult?.timeExpired) {
      return (
        <>
          <div style={{ fontSize: '4rem' }} className="mb-3">⏰</div>
          <h4 className="text-warning mb-3">Tempo Scaduto!</h4>
          <p className="text-muted">{lastGuessResult.message}</p>
        </>
      );
    }

    if (lastGuessResult?.isCorrect) {
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
          La posizione corretta era: {(lastGuessResult?.correctPosition || 0) + 1}
        </p>
      </>
    );
  }

  function RenderResultActions() {
    if (lastGuessResult?.isGameOver) {
      return (
        <div className="mt-4">
          <div className="d-grid gap-2">
            <button className="btn btn-primary" onClick={handleRestartGame}>
              🔄 Nuova Partita
            </button>
            <Link to="/" className="btn btn-outline-secondary">
              🏠 Torna alla Home
            </Link>
          </div>
        </div>
      );
    }

    return (
      <div className="mt-4">
        <button 
          className="btn btn-primary"
          onClick={handleNextAction}
        >
          🚀 Prossimo Round
        </button>
      </div>
    );
  }

  function RenderGameOverState() {
    return (
      <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '70vh' }}>
        <div className="card" style={{ width: '500px' }}>
          <div className="card-body text-center p-4">
            <h4 className="mb-3">Partita Terminata!</h4>
            <div className="mb-3">
              <div className="row text-center">
                <div className="col-4">
                  <div className="fw-bold fs-5 text-primary">{currentGame?.totalCards || 0}</div>
                  <small className="text-muted">Carte Vinte</small>
                </div>
                <div className="col-4">
                  <div className="fw-bold fs-5 text-danger">{currentGame?.wrongGuesses || 0}</div>
                  <small className="text-muted">Errori</small>
                </div>
                <div className="col-4">
                  <div className="fw-bold fs-5 text-success">{currentRound - 1}</div>
                  <small className="text-muted">Round Giocati</small>
                </div>
              </div>
            </div>
            <div className="d-grid gap-2">
              <button className="btn btn-primary" onClick={handleRestartGame}>
                🔄 Nuova Partita
              </button>
              <a href="/" className="btn btn-outline-secondary">
                🏠 Torna alla Home
              </a>
            </div>
          </div>
        </div>
      </div>
    );
  }
}

export default Game;