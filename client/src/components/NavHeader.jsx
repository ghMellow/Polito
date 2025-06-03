import { Container, Navbar } from 'react-bootstrap';
import { Link } from "react-router";

function NavHeader() {

  return(
    <Navbar bg='primary'>
      <Container fluid>
      <Link to="/" className="navbar-brand text-white">Shit Happens: The Game 🎮</Link>
      </Container>
    </Navbar>
  );
}

export default NavHeader;