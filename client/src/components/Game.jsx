
import { useState, useEffect } from 'react';
import API from '../API/API.mjs';

function Game({ loggedIn, user }) {
  // Stati del gioco
  const [gameState, setGameState] = useState('init');
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
    console.log('🎮 Game component mounted - initializing game');
    initializeGame();
    
    return () => {
      console.log('🎮 Game component unmounting');
    };
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
      setGameState('loading');
      
      // Crea una nuova partita
      const gameData = await API.createGame();
      console.log('Game data received:', gameData); // Debug log
      setCurrentGame(gameData);
      setPlayerCards(gameData.cards);
      
      // Inizia il primo round
      await startNewRound(gameData.gameId);
      
    } catch (error) {
      console.error('Errore nell\'inizializzazione del gioco:', error);
      setGameState('init');
    }
  };

  const startNewRound = async (gameId = currentGame?.gameId) => {
    try {
      setGameState('loading');
      
      // Ottieni carta per il nuovo round
      const roundData = await API.startNewRound(gameId);
      
      setTargetCard(roundData.card);
      setCurrentRound(roundData.roundNumber);
      setTimer(30);
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
      setGameState('waiting_result');
      
      const result = await API.submitGuess(
        currentGame.gameId,
        targetCard.id,
        position,
        currentRound
      );
      
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
    setGameState('init');
    setCurrentGame(null);
    setPlayerCards([]);
    setTargetCard(null);
    setCurrentRound(1);
    initializeGame();
  };

  const formatTime = (seconds) => {
    return `${seconds.toString().padStart(2, '0')}`;
  };

  if (gameState === 'init' || gameState === 'loading') {
    return <RenderLoadingState />;
  }

  if (gameState === 'game_over' && !showResultModal) {
    return <RenderGameOverState />;
  }

  return (
    <>
      <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '70vh' }}>
        <div style={{ width: '100%', maxWidth: '800px' }}>
          <RenderTargetCardSection />
          <RenderPlayerCardsSection />
        </div>
      </div>
      <RenderResultModal />
    </>
  );

  function RenderLoadingState() {
    return (
      <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '70vh' }}>
        <div className="card" style={{ width: '400px' }}>
          <div className="card-body text-center p-4">
            <h5>{gameState === 'init' ? 'Inizializzazione...' : 'Caricamento round...'}</h5>
            <div className="spinner-border text-primary mt-3" role="status">
              <span className="visually-hidden">Loading...</span>
            </div>
          </div>
        </div>
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

  function RenderTargetCardSection() {
    return (
      <div className="card mb-4">
        <div className="card-body p-4">
          <div className="row align-items-center">
            <div className="col-12 col-md-8 text-center mb-3 mb-md-0">
              <h5 className="mb-3">Trova questa carta tra le tue:</h5>
              <div 
                className="mx-auto bg-light border rounded d-flex align-items-center justify-content-center position-relative overflow-hidden"
                style={{ 
                  width: '120px', 
                  height: '180px'
                }}
              >
                {targetCard?.image_path ? (
                  <img 
                    src={API.getImage(targetCard.image_path)} 
                    alt={targetCard.text}
                    className="img-fluid h-100 w-100"
                    style={{ objectFit: 'cover' }}
                  />
                ) : (
                  <div className="text-muted">Caricamento...</div>
                )}
              </div>
            </div>
            <div className="col-12 col-md-4 text-center">
              <div className="mb-2">
                <small className="text-muted">Round {currentRound}</small>
              </div>
              <div className="text-dark" style={{ fontSize: '3rem', fontWeight: 'bold' }}>
                {formatTime(timer)}
              </div>
              <small className="text-muted">secondi rimasti</small>
            </div>
          </div>
        </div>
      </div>
    );
  }

  function RenderPlayerCardsSection() {
    return (
      <div className="card">
        <div className="card-header d-flex justify-content-between align-items-center">
          <h6 className="mb-0">🃏 Le tue carte - Seleziona la posizione</h6>
          <small className="text-muted">
            Carte: {currentGame?.totalCards || 0} | Errori: {currentGame?.wrongGuesses || 0}/3
          </small>
        </div>
        <div className="card-body p-4">
          <RenderCardsGrid />
          <RenderSelectedPositionInfo />
        </div>
      </div>
    );
  }

  function RenderCardsGrid() {
    return (
      <div className="d-flex justify-content-center align-items-center flex-wrap gap-2">
        {playerCards.map((card, index) => (
          <div key={card.id} className="d-flex align-items-center">
            <button
              className={`btn ${selectedPosition === index ? "btn-primary" : "btn-outline-secondary"} btn-sm me-2`}
              onClick={() => handlePositionSelect(index)}
              disabled={gameState !== 'playing'}
              style={{ minWidth: '40px' }}
            >
              {index + 1}
            </button>
            
            <div 
              className="bg-light border rounded d-flex align-items-center justify-content-center me-2 position-relative overflow-hidden"
              style={{ 
                width: '60px', 
                height: '90px'
              }}
            >
              <img 
                src={API.getImage(card.image_path)} 
                alt={card.text}
                className="img-fluid h-100 w-100"
                style={{ objectFit: 'cover' }}
              />
            </div>
          </div>
        ))}
        
        <button
          className={`btn ${selectedPosition === playerCards.length ? "btn-primary" : "btn-outline-secondary"} btn-sm`}
          onClick={() => handlePositionSelect(playerCards.length)}
          disabled={gameState !== 'playing'}
          style={{ minWidth: '40px' }}
        >
          {playerCards.length + 1}
        </button>
      </div>
    );
  }

  function RenderSelectedPositionInfo() {
    if (selectedPosition === null || gameState !== 'playing') {
      return null;
    }

    return (
      <div className="text-center mt-3">
        <small className="text-muted">
          Posizione selezionata: {selectedPosition + 1}
        </small>
        <br />
        <button 
          className="btn btn-success btn-sm mt-2"
          onClick={() => submitGuess(selectedPosition)}
        >
          ✅ Conferma Scelta
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
          <p className="text-muted">{lastGuessResult.message}</p>
        </>
      );
    }

    return (
      <>
        <div style={{ fontSize: '4rem' }} className="mb-3">😔</div>
        <h4 className="text-danger mb-3">Sbagliato!</h4>
        <p className="text-muted">
          La posizione corretta era: {(lastGuessResult?.correctPosition || 0) + 1}
          {lastGuessResult?.selectedPosition >= 0 && 
            ` (hai scelto: ${lastGuessResult.selectedPosition + 1})`
          }
        </p>
        <p className="text-muted">{lastGuessResult?.message}</p>
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
            <a href="/" className="btn btn-outline-secondary">
              🏠 Torna alla Home
            </a>
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
}

export default Game;