import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

import TasksList from "./components/TasksList";


function App() {
  return (
    <div>
      <h1>My To-Do App</h1>
      <TasksList />
    </div>
  );
}

export default App
