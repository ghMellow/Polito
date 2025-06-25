import { useState } from "react";
import { Routes, Route } from "react-router";

import DefaultLayout from "./components/DefaultLayout";
import AddTask from "./components/AddTask";
import TasksList from "./components/TasksList";
import EditTaskForm from "./components/editTask";


import "bootstrap/dist/css/bootstrap.min.css";
import 'bootstrap-icons/font/bootstrap-icons.css';


import { TaskProvider } from "./components/TaskContext";


function App() {
  return (
    <TaskProvider>
      <Routes>
        <Route element={<DefaultLayout />}>
        
          <Route path="/" element={<TasksList/>} />
          
          <Route path="/new" element={<AddTask/>} />

          <Route path="/edit/:id" element={<EditTaskForm/>} />

          <Route path="*" element={<p>Pagina non trovata</p>} />

        </Route>
      </Routes>
    </TaskProvider>
  );
}

export default App
