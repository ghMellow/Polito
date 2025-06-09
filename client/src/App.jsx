import "bootstrap/dist/css/bootstrap.min.css";
import { useEffect, useState } from "react";
import { Routes, Route, Navigate, useNavigate } from "react-router";

import DefaultLayout from "./components/DefaultLayout";
import NotFound from "./components/NotFound";
import Home from "./components/Home";
import Rules from "./components/Rules";
import Game from "./components/Game"; 
import GamesHistory from "./components/GamesHistory";
import GameDetails from "./components/GameDetails"; 
import GameSummary from "./components/GameSummary";
import { LoginForm } from "./components/AuthComponents";
import API from "./API/API.mjs";

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [message, setMessage] = useState('');
  const [user, setUser] = useState('');
  const navigate = useNavigate();


  useEffect(() => {
    const checkAuth = async () => {
      try {
        const user = await API.getUserInfo();
        setLoggedIn(true);
        setUser(user);
      } catch (error) {
        setLoggedIn(false);
        setUser(null);
      }
    };
    checkAuth();
  }, []);

  const handleLogin = async (credentials) => {
    try {
      const user = await API.logIn(credentials);
      setLoggedIn(true);
      setMessage({msg: `Benvenuto, ${user.username}!`, type: 'success'});
      setUser(user);
    }catch(err) {
      setMessage({msg: err, type: 'danger'});
    }
  };

  const handleLogout = async () => {
    await API.logOut();
    setLoggedIn(false);
    // clean up everything
    setMessage('');
    navigate('/');
  };

  return (
    <Routes>
      <Route element={ <DefaultLayout loggedIn={loggedIn} handleLogout={handleLogout} message={message} setMessage={setMessage} /> } >
        <Route path="/" element={<Home loggedIn={loggedIn} user={user} handleLogout={handleLogout} />} />
        <Route path='/login' element={loggedIn ? <Navigate replace to='/' /> : <LoginForm handleLogin={handleLogin} />} />
        <Route path="/rules" element={<Rules />} />
        <Route path="/game" element={<Game loggedIn={loggedIn}/>} />
        <Route path="/history" element={<GamesHistory loggedIn={loggedIn} user={user} handleLogout={handleLogout} />} />
        <Route path="/history/:gameId" element={<GameDetails loggedIn={loggedIn} user={user} handleLogout={handleLogout} />} />
        <Route path="/summary" element={<GameSummary loggedIn={loggedIn} user={user} />} />
        <Route path="*" element={ <NotFound /> } />
      </Route>
    </Routes>
  )
}

export default App
