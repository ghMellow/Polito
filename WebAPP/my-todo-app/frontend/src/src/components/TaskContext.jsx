// context/TaskContext.js
import { createContext, useContext, useState, useEffect } from "react";

const TaskContext = createContext();

export function TaskProvider({ children }) {
  const [tasks, setTasks] = useState([]);

  const fetchTasks = () => {
    fetch("http://localhost:3001/api/tasks")
      .then(res => res.json())
      .then(data => setTasks(data))
      .catch(err => console.error("Fetch error:", err));
  };

  useEffect(() => {
    fetchTasks();
  }, []);

  return (
    <TaskContext.Provider value={{ tasks, setTasks, fetchTasks }}>
      {children}
    </TaskContext.Provider>
  );
}

export function useTasks() {
  return useContext(TaskContext);
}
