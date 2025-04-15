import { useState } from "react";
import { Routes, Route } from "react-router";

import DefaultLayout from "./components/DefaultLayout";
import AddTask from "./components/AddTask";
import TasksList from "./components/TasksList";

import "bootstrap/dist/css/bootstrap.min.css";
import 'bootstrap-icons/font/bootstrap-icons.css';


function App() {
  return (
    <Routes>
      <Route element={<DefaultLayout />}>
      
        <Route path="/" element={<TasksList/>} />
        
        <Route path="/new" element={<AddTask/>} />

        <Route path="*" element={<p>Pagina non trovata</p>} />

      </Route>
    </Routes>
  );
}

export default App
