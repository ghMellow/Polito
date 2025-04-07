import { useEffect, useState } from "react";

function TasksList() {
  const [tasks, setTasks] = useState([]);

  const fetchTasks = () => {
    fetch("http://localhost:3001/api/tasks")
      .then(response => response.json())
      .then(data => {
        setTasks(data);
      })
      .catch(error => {
        console.error("Errore nel recupero dei task:", error);
      });
  };

  useEffect(() => {
    fetchTasks();
  }, []);

  const handleDelete = (id) => {
    fetch(`http://localhost:3001/api/tasks/${id}`, {
      method: "DELETE",
    })
      .then(response => {
        if (response.status === 204) {
          // Rimuovi il task dalla lista locale
          setTasks(tasks.filter(task => task.id !== id));
        } else {
          console.error("Errore nell'eliminazione del task");
        }
      })
      .catch(error => {
        console.error("Errore nell'eliminazione del task:", error);
      });
  };

  const handleToggleComplete = (id, currentStatus) => {
    fetch(`http://localhost:3001/api/tasks/${id}/toggle`, {
      method: "PATCH",
    })
      .then(response => response.json())
      .then(updatedTask => {
        // Aggiorna lo stato locale con il task aggiornato
        setTasks(tasks.map(task => 
          task.id === id ? updatedTask : task
        ));
      })
      .catch(error => {
        console.error("Errore nell'aggiornamento del task:", error);
      });
  };

  return (
    <div className="w-full pl-4 pr-4 pt-2 pb-4">
      <h1 className="form-title" align="left">To-Do List</h1>
      <table className="task-table">
        <thead>
          <tr>
            <th>Completato</th>
            <th>ID</th>
            <th>Task</th>
            <th>Utente ID</th>
            <th>Scadenza</th>
            <th>Priorità</th>
            <th>Data Creazione</th>
            <th>Azioni</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.id}>
              <td>
                <input
                  type="checkbox"
                  checked={task.completed}
                  onChange={() => handleToggleComplete(task.id, task.completed)}
                  className="form-checkbox"
                />
              </td>
              <td>{task.id}</td>
              <td>{task.text}</td>
              <td>{task.userId || "N/A"}</td>
              <td>{task.dueDate || "N/A"}</td>
              <td>{task.priority}</td>
              <td>{task.createdAt}</td>
              <td>
                <button
                  onClick={() => handleDelete(task.id)}
                  className="bg-red-500 text-white px-3 py-1 rounded hover:bg-red-600"
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default TasksList;