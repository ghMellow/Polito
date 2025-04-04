// NavHeader.jsx
import logo from '../assets/todo.svg';

/* 
- display: flex → Dispone gli elementi figli sulla stessa riga.
- alignItems: "center" → Li allinea verticalmente al centro.
- marginLeft: "8px" → Dà un po' di spazio tra il logo e il testo.
 */

function NavHeader() {
  return (
    <div style={{ display: "flex", alignItems: "center" }}>
      <img
        alt="Todo Logo"
        src={logo}
        width="50"
        height="50"
      />
      <h1 style={{ marginLeft: "8px" }}>Todo App</h1>
    </div>
  );
}

export default NavHeader;