import { useState, useEffect } from 'react';
import guestImg from '../assets/guest.png';
import { Card, Row, Col, Badge, Spinner, Alert } from 'react-bootstrap';
import { Link } from 'react-router';
import { LogoutButton } from './AuthComponents';
import API from '../API/API.mjs';
import dayjs from 'dayjs';

function GamesHistory(props) {
  const [userProfile, setUserProfile] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchUserProfile = async () => {
      if (props.loggedIn) {
        try {
          setLoading(true);
          const profile = await API.getUserProfile();
          setUserProfile(profile);
        } catch (err) {
          setError('Errore nel caricamento dello storico partite');
          console.error('Errore nel caricamento del profilo:', err);
        } finally {
          setLoading(false);
        }
      } else {
        setLoading(false);
      }
    };

    fetchUserProfile();
  }, [props.loggedIn]);

  const formatDate = (dateString) => {
    return dayjs(dateString).format('DD MMMM YYYY, HH:mm');
  };

  const getStatusBadge = (status) => {
    return status === 'won' ? 
      <Badge bg="success">Vinta 🏆</Badge> : 
      <Badge bg="danger">Persa 😢</Badge>;
  };

  if (!props.loggedIn) {
    return (
      <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '55vh' }}>
        <Alert variant="warning" className="text-center">
          <h4>Accesso Richiesto</h4>
          <p>Devi effettuare il login per visualizzare lo storico delle partite.</p>
          <Link to="/login" className="btn btn-primary">
            Vai al Login
          </Link>
        </Alert>
      </div>
    );
  }

  return (
    <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '55vh' }}>
      <div style={{ width: '100%', maxWidth: '800px' }}>
        <RenderUserProfileCard />
        <RenderGamesHistorySection />
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
                <img 
                  src={guestImg}
                  alt="User profile" 
                  className="w-100 h-100"
                  style={{ objectFit: 'cover' }}
                />
              </div>
            </Col>
            <Col>
              <div className="d-flex justify-content-between align-items-start">
                <div className="flex-grow-1 me-3">
                  <h4 className="mb-2">{props.user?.username || 'Utente'}</h4>
                  <p className="text-muted mb-3">{props.user?.email || ''}</p>
                  <Link 
                    to="/" 
                    className="btn btn-outline-info text-muted btn-sm d-inline-flex align-items-center"
                  >
                    🏠 Torna alla Home
                  </Link>
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

  function RenderGamesHistorySection() {
    return (
      <Card>
        <Card.Header>
          <h5 className="mb-0">📜 Storico Partite</h5>
        </Card.Header>
        <Card.Body>
          {loading ? (
            <RenderLoadingState />
          ) : error ? (
            <Alert variant="danger">{error}</Alert>
          ) : userProfile?.history?.length > 0 ? (
            <RenderGamesList />
          ) : (
            <RenderEmptyState />
          )}
        </Card.Body>
      </Card>
    );
  }

  function RenderLoadingState() {
    return (
      <div className="text-center py-4">
        <Spinner animation="border" role="status">
          <span className="visually-hidden">Caricamento...</span>
        </Spinner>
      </div>
    );
  }

  function RenderGamesList() {
    return (
      <div className="row g-3">
        {userProfile.history.map((game) => (
          <div key={game.id} className="col-12">
            <Card className="h-100">
              <Card.Body>
                <div className="d-flex justify-content-between align-items-start mb-3">
                  <div>
                    <h6 className="mb-1">Partita #{game.id}</h6>
                    <small className="text-muted">
                      Iniziata: {formatDate(game.created_at)}
                    </small>
                    {game.completed_at && (
                      <><br />
                        <small className="text-muted">
                          Completata: {formatDate(game.completed_at)}
                        </small>
                      </>
                    )}
                  </div>
                  {getStatusBadge(game.status)}
                </div>
                
                <Row className="text-center">
                  <Col xs={4}>
                    <div className="border-end">
                      <div className="fw-bold fs-4 text-primary">{game.total_cards}</div>
                      <small className="text-muted">Carte Totali</small>
                    </div>
                  </Col>
                  <Col xs={4}>
                    <div className="border-end">
                      <div className="fw-bold fs-4 text-danger">{game.wrong_guesses}</div>
                      <small className="text-muted">Errori</small>
                    </div>
                  </Col>
                  <Col xs={4}>
                    <div className="fw-bold fs-4 text-success">
                      {game.cards?.filter(card => card.won === 1).length || 0}
                    </div>
                    <small className="text-muted">Carte Vinte</small>
                  </Col>
                </Row>

                <div className="mt-3 d-flex justify-content-between align-items-center">
                  <Link 
                    to={`/history/${game.id}`}
                    className="btn btn-outline-primary btn-sm"
                  >
                    🔍 Vedi Dettagli
                  </Link>
                  <small className="text-muted">
                    {game.cards?.length || 0} carte in totale
                  </small>
                </div>
              </Card.Body>
            </Card>
          </div>
        ))}
      </div>
    );
  }

  function RenderEmptyState() {
    return (
      <div className="text-center py-4">
        <h6 className="text-muted">Nessuna partita giocata ancora</h6>
        <p className="text-muted mb-3">Inizia la tua prima partita!</p>
        <Link to="/" className="btn btn-primary">
          🕹️ Gioca Ora
        </Link>
      </div>
    );
  }
}

export default GamesHistory;