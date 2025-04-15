import { Outlet } from "react-router";
import NavHeader from "./NavHeader";
import { Container } from "react-bootstrap";

function DefaultLayout() {

  return(
    <>
      <NavHeader />
      <br></br>
      <Container fluid className="mt-3">
        <Outlet />
      </Container>
    </>
  );

}

export default DefaultLayout;