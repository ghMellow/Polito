import { useState, useEffect } from 'react';
import { useParams, useLocation, Link } from 'react-router';
import { Card, Row, Col, Badge } from 'react-bootstrap';
import { LogoutButton } from './AuthComponents';
import GameAPI from '../API/gameAPI';
import dayjs from 'dayjs';
import UserAvatar from './UserAvatar';

function GameDetails(props) {
  const { gameId } = useParams();
  const location = useLocation();
  const [gameData, setGameData] = useState();

  useEffect(() => {
    const fetchGameDetails = async () => {
      if (location.state?.gameData) {
        const passedGame = location.state.gameData;
        if (passedGame.id === parseInt(gameId)) {
          setGameData(passedGame);
          return;
        } else {
          throw new Error('Game data mismatch: passed game id does not match URL game id.');
        }
      } else {
        throw new Error('No location state provided with game data.');
      }
    };

    fetchGameDetails();
  }, [gameId, props.loggedIn, location.state]);

  const formatDate = (dateString) => {
    return dayjs(dateString).format('DD MMMM YYYY, HH:mm');
  };

  const getStatusBadge = (status) => {
    return status === 'won' ?
      <Badge bg="success">Vinta 🏆</Badge> : <Badge bg="danger">Persa 😢</Badge>;
  };

  const getCardStatusBadge = (card) => {
    if (card.initial_card) {
      return <Badge bg="info">Carta Iniziale</Badge>;
    }
    return card.won ?
      <Badge bg="success">Vinta ✅</Badge> :
      <Badge bg="danger">Persa ❌</Badge>;
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
              <UserAvatar />
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
                      ↩️ Indietro
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
              <small className="text-muted d-block">Data:</small>
              <strong>{formatDate(gameData.created_at)}</strong>
            </Col>
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
              <div className="fw-bold fs-3 text-success">{gameData.correct_guesses}</div>
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
          <h5 className="mb-0">🃏 Carte della Partita</h5>
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
          .sort((a, b) => a.round_number - b.round_number)
          .map((card) => (
            <Col key={card.id} md={6} lg={4}>
              <Card className="h-100 border-2" >

                <Card.Body className="d-flex flex-column">
                  <div className="d-flex justify-content-between align-items-start mb-2">
                    {card.round_number ? (<small className="text-muted">Round #{card.round_number}</small>) : ''}
                    {getCardStatusBadge(card)}
                  </div>

                  <Card.Text className="flex-grow-1">
                    {card.text}
                  </Card.Text>

                  {card.image_path && (
                    <img
                      src={GameAPI.getImage(card.image_path)}
                      alt="Card image"
                      className="img-fluid mb-3"
                      style={{
                        width: '100%',
                        height: 'auto',
                        objectFit: 'contain'
                      }}
                      onError={(e) => {
                        e.target.style.display = 'none';
                      }}
                    />
                  )}

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