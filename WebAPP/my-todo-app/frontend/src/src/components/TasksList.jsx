import { useEffect, useState } from "react";

function TasksList() {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    fetch("http://localhost:3001/api/tasks")
      .then(response => response.json())
      .then(data => {
        setTasks(data);
      })
      .catch(error => {
        console.error("Errore nel recupero dei task:", error);
      });
  }, []);

  return (
    <div className="w-full pl-4 pr-4 pt-2 pb-4">
      <h2 align="left">To-Do List</h2>
      <table className="task-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Task</th>
            <th>Completato</th>
            <th>Utente ID</th>
            <th>Scadenza</th>
            <th>Priorità</th>
            <th>Data Creazione</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.id}>
              <td>{task.id}</td>
              <td>{task.text}</td>
              <td>{task.completed ? "Sì" : "No"}</td>
              <td>{task.userId || "N/A"}</td>
              <td>{task.dueDate || "N/A"}</td>
              <td>{task.priority}</td>
              <td>{task.createdAt}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default TasksList;