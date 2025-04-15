// NavHeader.jsx
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Navbar from 'react-bootstrap/Navbar';

import logo from '../assets/todo.svg';


function NavHeader() {
  return (
    <Navbar bg="light">
      <Container className="justify-content-md-center">
        <Navbar.Brand>
          <Row>
            <Col>
              <img
                alt=""
                src={logo}
                width="30"
                height="30"
                className="d-inline-block align-top"
              />{' '}
            </Col>
            <Col>
              <h4>ToDo App</h4>
            </Col>
          </Row>
        </Navbar.Brand>
      </Container>
    </Navbar>
  );
}

export default NavHeader;