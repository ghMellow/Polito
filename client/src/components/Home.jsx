import guestImg from '../assets/guest.png';
import { Card, Button, Row, Col } from 'react-bootstrap';
import { Link } from 'react-router';
import { LogoutButton } from './AuthComponents';

function Home(props) {
  return (
    <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '55vh' }}>
      <div style={{ width: '100%', maxWidth: '600px' }}>
        {/* Card profilo utente */}
        <Card className="mb-4">
          <Card.Body className="p-4">
            <Row className="align-items-center">
              <Col xs="auto">
                {/* Immagine profilo placeholder */}
                <div 
                    className="rounded-circle d-flex align-items-center justify-content-center overflow-hidden"
                    style={{ width: '120px', height: '120px' }}
                >
                    {props.loggedIn ? (
                        <img 
                        src={guestImg}
                        alt="User profile" 
                        className="w-100 h-100"
                        style={{ objectFit: 'cover' }}
                        />
                        ) : (
                        <img 
                            src={guestImg}
                            alt="Utente ospite" 
                            className="w-100 h-100"
                            style={{ objectFit: 'cover' }}
                        />
                        )}
                </div>
              </Col>
              <Col>
                <div className="d-flex justify-content-between align-items-start">
                  <div className="flex-grow-1 me-3">
                    {props.loggedIn ? (
                      <>
                        <h4 className="mb-2">Benvenuto, {props.user.username}!</h4>
                        <p className="text-muted mb-3">{props.user.email}</p>
                        {/* Link allo storico partite */}
                        <Link 
                          to="/history" 
                          className="btn btn-outline-info text-muted btn-sm d-inline-flex align-items-center"
                        >
                          Storico partite 📜 
                        </Link>
                      </>
                    ) : (
                      <>
                        <h4 className="mb-2">Ospite</h4>
                        <p className="text-muted mb-0">Fai il login per sbloccare tutte le funzionalità!</p>
                      </>
                    )}
                  </div>
                  <div className="flex-shrink-0">
                    {props.loggedIn ? (
                      <LogoutButton logout={props.handleLogout} />
                    ) : (
                      <Link to="/login" className="btn btn-primary">
                        Login
                      </Link>
                    )}
                  </div>
                </div>
              </Col>
            </Row>
          </Card.Body>
        </Card>

        {/* Bottoni principali */}
        <Row className="justify-content-center mb-3">
          <Col xs={8} sm={6}>
            <Link 
              to="/rules" 
              // className="btn btn-outline-info text-muted btn-sm d-inline-flex align-items-center"
              className="btn btn-outline-primary w-100 d-flex align-items-center justify-content-center text-decoration-none"
              style={{ height: '60px' }}
              >
              <div className="text-center fs-5">
                📖 Regole
              </div>
            </Link>
          </Col>
        </Row>
        <Row className="justify-content-center">
          <Col xs={8} sm={6}>
            <Link 
              to={props.loggedIn ? "/game" : "/demo"}
              className={`btn ${props.loggedIn ? "btn-success" : "btn-secondary"} w-100 d-flex align-items-center justify-content-center text-decoration-none`}
              style={{ height: '60px' }}
            >
              <div className="text-center fs-5">
                {props.loggedIn ? ' Gioca 🕹️' : ' Demo 🎯'}
              </div>
            </Link>
          </Col>
        </Row>
      </div>
    </div>
  );
}

export default Home;