import { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router';
import { Card, Badge } from 'react-bootstrap';
import API from '../API/API.mjs';

function GameSummary() {
  const location = useLocation();
  const [playerCards, setPlayerCards] = useState([]);

  useEffect(() => {
    if (location.state) {
      setPlayerCards(location.state.playerCards || []);
    }
  }, [location.state]);

  return (
    <div className="container mt-4">
      <div className="text-center mb-4">
        <h2>Riepilogo</h2>
      </div>
      
      <RenderCardsListSection />

      <div className="text-center mt-4">
        <div className="d-flex gap-2 justify-content-center">
          <Link to="/game" className="btn btn-primary">
            🔄 Gioca Ancora
          </Link>
          <Link to="/" className="btn btn-outline-secondary">
            🏠 Home
          </Link>
        </div>
      </div>
    </div>
  );

  function RenderCardsListSection() {
    return (
      <Card className="h-100" style={{ maxWidth: '100%' }}>
        <Card.Body className="d-flex flex-column h-100">
          <div className="d-flex justify-content-between align-items-center mb-3">
            <h6 className="mb-0">🃏 Le tue carte</h6>
          </div>
          
          <div className="w-100 overflow-auto flex-grow-1 d-flex align-items-center justify-content-center">
            <RenderCardsGrid />
          </div>
        </Card.Body>
      </Card>
    );
  }

  function RenderCardsGrid() {
    return (
      <div className="d-flex justify-content-center align-items-center flex-nowrap gap-2 overflow-auto">
        {playerCards.map((card, index) => (
          <div key={card.id} className="d-flex align-items-center flex-shrink-0">
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
      </div>
    );
  }
}

export default GameSummary;