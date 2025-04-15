import { useEffect, useState } from "react";
import { useNavigate } from 'react-router-dom';
import {Table, Row, Col, Button, Container} from 'react-bootstrap';


function TasksList() {
  const navigate = useNavigate();
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
    // Carica i task all'inizio
    // Fetch tasks only once when the component mounts
    fetchTasks();
    
    // Ricarica quando la finestra ottiene il focus (dopo essere tornati da altre pagine)
    const handleFocus = () => {
      fetchTasks();
    };
    
    window.addEventListener('focus', handleFocus);
    
    // Cleanup
    return () => {
      window.removeEventListener('focus', handleFocus);
    };
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

  const handleToggleComplete = (id) => {
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
    <Container className="justify-content-md-center">
      <Row>
        <Col className="flex-grow-1">
          <h1>ToDo List</h1>
        </Col>
        <Col xs="auto" className="pe-4">
          <Button variant="primary" onClick={() => navigate('/new')}>
            Add new task
          </Button>
        </Col>
      </Row>
      <Row className="my-3">
        <TaskTable 
          tasks={tasks} 
          handleToggleComplete={handleToggleComplete} 
          handleDelete={handleDelete} 
        />
      </Row>
    </Container>
  );
}

// Gestisce la tabella e la logica di ordinamento
function TaskTable(props) {
  const [sortOrder, setSortOrder] = useState("none");
  const [sortField, setSortField] = useState("id");

  const sortedTasks = [...props.tasks];
  if (sortOrder !== "none") {
    sortedTasks.sort((a, b) => {
      let comparison = 0;
      if (a[sortField] < b[sortField]) comparison = -1;
      if (a[sortField] > b[sortField]) comparison = 1;
      return sortOrder === "asc" ? comparison : -comparison;
    });
  }

  const handleSort = (field) => {
    if (sortField === field) {
      setSortOrder(oldOrder => oldOrder === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortOrder("asc");
    }
  };

  const getSortIcon = (field) => {
    if (sortField !== field) return "bi bi-arrow-down-up";
    return sortOrder === "asc" ? "bi bi-sort-down" : "bi bi-sort-up";
  };

  return (
    <Table responsive striped>
      <thead>
        <tr>
          <th><i class="bi bi-bookmark-check"></i></th>
          <th onClick={() => handleSort("id")}>
            ID <i className={getSortIcon("id")}></i>
          </th>
          <th onClick={() => handleSort("text")}>
            Task <i className={getSortIcon("text")}></i>
          </th>
          <th onClick={() => handleSort("userId")}>
            User ID <i className={getSortIcon("userId")}></i>
          </th>
          <th onClick={() => handleSort("dueDate")}>
            Scadenza <i className={getSortIcon("dueDate")}></i>
          </th>
          <th onClick={() => handleSort("priority")}>
            Priorità <i className={getSortIcon("priority")}></i>
          </th>
          <th onClick={() => handleSort("createdAt")}>
            Data Creazione <i className={getSortIcon("createdAt")}></i>
          </th>
          <th>Azioni</th>
        </tr>
      </thead>
      <tbody>
        {sortedTasks.map((task) => (
          <TaskRow 
            key={task.id} 
            task={task} 
            handleToggleComplete={props.handleToggleComplete} 
            handleDelete={props.handleDelete} 
          />
        ))}
      </tbody>
    </Table>
  );
}

// Rappresenta una singola riga della tabella
function TaskRow(props) {
  return (
    // className="table-success" è una classe boostrap che colore di verde
    <tr className={props.task.completed ? "table-success" : ""}>
      <TaskData task={props.task} handleToggleComplete={props.handleToggleComplete} />
      <TaskAction task={props.task} handleDelete={props.handleDelete} />
    </tr>
  );
}

// Contiene i dati di un task
function TaskData(props) {
  return (
    <>
      <td>
        <input 
          className="form-check-input" 
          type="checkbox" 
          checked={props.task.completed}
          onChange={() => props.handleToggleComplete(props.task.id)} 
          aria-label="Checkbox for following text input"
        />
      </td>
      <td>{props.task.id}</td>
      <td>{props.task.text}</td>
      <td>{props.task.userId || "N/A"}</td>
      <td>{props.task.dueDate || "N/A"}</td>
      <td>{props.task.priority}</td>
      <td>{props.task.createdAt}</td>
    </>
  );
}

// Contiene i pulsanti di azione
function TaskAction(props) {
  return (
    <td>
      <Button 
        variant="primary"
        onClick={() => props.handleDelete(props.task.id)}
      >
        <i className="bi bi-trash" />
      </Button>
    </td>
  );
}

export default TasksList;