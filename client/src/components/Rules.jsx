import { Card } from 'react-bootstrap';
import { Link } from 'react-router';

function Rules() {
    return (
        <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '50vh' }}>
            <div style={{ width: '100%', maxWidth: '600px' }}>
                <Card>
                    <Card.Body>
                        <Card.Title className="text-center mb-4">Regole del Gioco 🎲</Card.Title>
                        <Card.Text>
                            Benvenuto al <strong>"Gioco della Sfortuna a tema Università"</strong>! 😱 In questa versione single player ispirata a "Stuff Happens", sfiderai il computer cercando di collezionare 6 carte che rappresentano situazioni orribili della vita universitaria... davvero orribili! 🫣
                        </Card.Text>
                        <Card.Text>
                            🔹 Inizia la partita con 3 carte casuali, ognuna con:
                        </Card.Text>
                        <ul>
                            <li>Un nome</li>
                            <li>Un'immagine</li>
                            <li>Un indice di sfortuna da 1 a 100 (più alto = peggio!)</li>
                        </ul>
                        <Card.Text>
                            🔸 A ogni round riceverai una nuova situazione misteriosa. Vedi solo il nome e l'immagine... ma non il suo livello di sfortuna! 😬
                        </Card.Text>
                        <Card.Text>
                            📍 Dovrai indovinare dove si colloca, per indice di sfortuna, tra le carte che già possiedi. Se indovini:
                        </Card.Text>
                        <ul>
                            <li>Hai 30 secondi ⏱️</li>
                            <li>La carta viene aggiunta alla tua collezione</li>
                        </ul>
                        <Card.Text>
                            ❌ Se sbagli o scade il tempo, la perdi per sempre!
                        </Card.Text>
                        <Card.Text>
                            🏁 La partita termina quando:
                        </Card.Text>
                        <ul>
                            <li>Hai raccolto 6 carte (🎉 hai vinto!)</li>
                            <li>Hai sbagliato 3 volte (😢 hai perso!)</li>
                        </ul>
                        <Card.Text>
                            👤 Utenti registrati possono giocare partite complete e vedere la loro cronologia. 👀
                        </Card.Text>
                        <Card.Text>
                            🕹️ Visitatori anonimi? Possono fare solo una partita demo di un round.
                        </Card.Text>
                        <Card.Text>
                            Buona fortuna... ne avrai bisogno! 🍀
                        </Card.Text>
                    </Card.Body>
                </Card>

                <div className="mt-4">
                    <Link 
                        to="/" 
                        className="btn btn-outline-primary w-100 d-flex align-items-center justify-content-center"
                        style={{ height: '60px' }}
                    >
                        🏠 Torna alla Home
                    </Link>
                </div>
            </div>
        </div>
    );
}

export default Rules;