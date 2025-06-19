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
import UserAPI from './API/userAPI';

function App() {
  const [loggedIn, setLoggedIn] = useState(false);
  const [message, setMessage] = useState('');
  const [user, setUser] = useState('');
  const navigate = useNavigate();


  useEffect(() => {
    const checkAuth = async () => {
      try {
        const user = await UserAPI.getUserInfo();
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
      const user = await UserAPI.logIn(credentials);
      setLoggedIn(true);
      setMessage({msg: `Benvenuto, ${user.username}!`, type: 'success'});
      setUser(user);
    }catch(err) {
      setMessage({msg: err, type: 'danger'});
    }
  };

  const handleLogout = async () => {
    await UserAPI.logOut();
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
        <Route path="/history" element={<GamesHistory loggedIn={loggedIn} user={user} handleLogout={handleLogout} />} />
        <Route path="/history/:gameId" element={<GameDetails loggedIn={loggedIn} user={user} handleLogout={handleLogout} />} />
        <Route path="/game" element={<Game loggedIn={loggedIn}/>} />
        <Route path="/summary" element={<GameSummary />} />
        <Route path="*" element={ <NotFound /> } />
      </Route>
    </Routes>
  )
}

export default App
