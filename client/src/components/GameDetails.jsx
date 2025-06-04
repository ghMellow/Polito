import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router';
import { Card, Row, Col, Badge } from 'react-bootstrap';
import { LogoutButton } from './AuthComponents';
import API from '../API/API.mjs';
import dayjs from 'dayjs';
import JdenticonAvatar from './JdenticonAvatar';

function GameDetails(props) {
  const { gameId } = useParams();
  const [gameData, setGameData] = useState();

  useEffect(() => {
    const fetchGameDetails = async () => {
      if (props.loggedIn) {
        try {
          const profile = await API.getUserProfile();
          
          const game = profile.history.find(g => g.id === parseInt(gameId));
          setGameData(game);
        } catch (err) {
          console.error('Errore nel recupero dei dettagli della partita:', err);
          setGameData(null);
        }
      }
    };

    fetchGameDetails();
  }, [gameId]);

  const formatDate = (dateString) => {
    return dayjs(dateString).format('DD MMMM YYYY, HH:mm');
  };

  const getStatusBadge = (status) => {
    return status === 'won' ? 
      <Badge bg="success">Vinta 🏆</Badge> : 
      <Badge bg="danger">Persa 😢</Badge>;
  };

  const getCardStatusBadge = (card) => {
    if (card.initial_card) {
      return <Badge bg="info">Carta Iniziale</Badge>;
    }
    return card.won ? 
      <Badge bg="success">Vinta ✅</Badge> : 
      <Badge bg="danger">Persa ❌</Badge>;
  };

  const getRoundText = (card) => {
    if (card.initial_card) {
      return 'Iniziale';
    }
    return card.round_number ? `Round ${card.round_number}` : 'N/A';
  };

  return (
    <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '55vh' }}>
      <div style={{ width: '100%', maxWidth: '900px' }}>
        <RenderUserProfileCard />
        <RenderGameDetailsSection />
      </div>
    </div>
  );

  function RenderUserProfileCard() {
    return (
      <Card className="mb-4">
        <Card.Body className="p-4">
          <Row className="align-items-center">
            <Col xs="auto">
              <div 
                className="rounded-circle d-flex align-items-center justify-content-center overflow-hidden"
                style={{ width: '120px', height: '120px' }}
              >
                <JdenticonAvatar 
                  value={props.user.username || Math.random().toString(36).substring(2, 15)}  
                  circular={true}
                />
              </div>
            </Col>
            <Col>
              <div className="d-flex justify-content-between align-items-start">
                <div className="flex-grow-1 me-3">
                  <h4 className="mb-2">{props.user?.username || 'Utente'}</h4>
                  <p className="text-muted mb-3">{props.user?.email || ''}</p>
                  <div className="d-flex gap-2">
                    <Link 
                      to="/" 
                      className="btn btn-outline-info text-muted btn-sm d-inline-flex align-items-center"
                    >
                      🏠 Home
                    </Link>
                    <Link 
                      to="/history" 
                      className="btn btn-outline-secondary text-muted btn-sm d-inline-flex align-items-center"
                    >
                      📜 Storico partite
                    </Link>
                  </div>
                </div>
                <div className="flex-shrink-0">
                  <LogoutButton logout={props.handleLogout} />
                </div>
              </div>
            </Col>
          </Row>
        </Card.Body>
      </Card>
    );
  }

  function RenderGameDetailsSection() {
    if (!gameData) return null;

    return (
      <>
        <RenderGameSummaryCard />
        <RenderCardsListSection />
      </>
    );
  }

  function RenderGameSummaryCard() {
    return (
      <Card className="mb-4">
        <Card.Header>
          <div className="d-flex justify-content-between align-items-center">
            <h5 className="mb-0">🎮 Partita #{gameData.id}</h5>
            {getStatusBadge(gameData.status)}
          </div>
        </Card.Header>
        <Card.Body>
          <Row className="mb-3">
            <Col md={3}>
              <small className="text-muted d-block">Iniziata:</small>
              <strong>{formatDate(gameData.created_at)}</strong>
            </Col>
            {gameData.completed_at && (
              <Col md={3}>
                <small className="text-muted d-block">Completata:</small>
                <strong>{formatDate(gameData.completed_at)}</strong>
              </Col>
            )}
          </Row>
          
          <Row className="text-center">
            <Col xs={4}>
              <div className="border-end">
                <div className="fw-bold fs-3 text-primary">{gameData.total_cards}</div>
                <small className="text-muted">Carte Totali</small>
              </div>
            </Col>
            <Col xs={4}>
              <div className="border-end">
                <div className="fw-bold fs-3 text-danger">{gameData.wrong_guesses}</div>
                <small className="text-muted">Errori</small>
              </div>
            </Col>
            <Col xs={4}>
              <div className="fw-bold fs-3 text-success">
                {gameData.cards?.filter(card => card.won === 1).length || 0}
              </div>
              <small className="text-muted">Carte Vinte</small>
            </Col>
          </Row>
        </Card.Body>
      </Card>
    );
  }

  function RenderCardsListSection() {
    return (
      <Card>
        <Card.Header>
          <h5 className="mb-0">🃏 Carte della Partita ({gameData.cards?.length || 0})</h5>
        </Card.Header>
        <Card.Body>
        <RenderCardsList />
        </Card.Body>
      </Card>
    );
  }

  function RenderCardsList() {
    return (
      <Row className="g-3">
        {gameData.cards
          .sort((a, b) => a.misfortune_index - b.misfortune_index)
          .map((card, index) => (
          <Col key={card.id} md={6} lg={4}>
            <Card className="h-100 border-2" style={{ 
              borderColor: card.won ? '#198754' : card.initial_card ? '#0dcaf0' : '#dc3545' 
            }}>
              {card.image_path && (
                <Card.Img 
                  variant="top" 
                  src={`http://localhost:3001${card.image_path}`}
                  style={{ height: '150px', objectFit: 'cover' }}
                  onError={(e) => {
                    e.target.style.display = 'none';
                  }}
                />
              )}
              <Card.Body className="d-flex flex-column">
                <div className="d-flex justify-content-between align-items-start mb-2">
                  <small className="text-muted">#{index + 1}</small>
                  {getCardStatusBadge(card)}
                </div>
                
                <Card.Text className="flex-grow-1">
                  {card.text}
                </Card.Text>
                
                <div className="mt-auto">
                  <div className="d-flex justify-content-between align-items-center mb-2">
                    <small className="text-muted">Indice Sfortuna:</small>
                    <Badge 
                      bg={card.misfortune_index > 70 ? 'danger' : 
                          card.misfortune_index > 40 ? 'warning' : 'success'}
                      className="fs-6"
                    >
                      {card.misfortune_index}
                    </Badge>
                  </div>
                  
                  <div className="d-flex justify-content-between align-items-center">
                    <small className="text-muted">Round:</small>
                    <Badge bg="secondary">{getRoundText(card)}</Badge>
                  </div>
                </div>
              </Card.Body>
            </Card>
          </Col>
        ))}
      </Row>
    );
  }
}

export default GameDetails;