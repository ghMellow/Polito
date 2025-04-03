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
      <h2>To-Do List</h2>
      <ul>
        {tasks.map((task) => (
          <li key={task.id}>{task.text}</li>
        ))}
      </ul>
    </div>
  );
}

export default TasksList;