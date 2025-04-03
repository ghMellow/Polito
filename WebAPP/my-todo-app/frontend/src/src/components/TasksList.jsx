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
    <div>
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
              <td>{task.user_id || "N/A"}</td>
              <td>{task.due_date || "N/A"}</td>
              <td>{task.priority}</td>
              <td>{task.created_at}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default TasksList;