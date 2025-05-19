import { useActionState } from "react";
import { Container, Form, Button, Row, Col, Alert } from 'react-bootstrap';
import { useNavigate } from 'react-router-dom';
import { Link } from 'react-router-dom';
import dayjs from 'dayjs';

function AddTask() {
  const navigate = useNavigate();

  const initialState = {
    text: '',
    priority: '1',
    due_date: dayjs().add(1, 'day').format('YYYY-MM-DD'),
    completed: false
  };
  
  const handleSubmit = async (prevState, formData) => {
    // Creazione oggetto dai dati del form
    const taskData = Object.fromEntries(formData.entries());
    
    // Gestione checkbox (non viene incluso in formData se non è selezionato)
    taskData.completed = formData.has('completed');
    
    // Validazione
    if(taskData.text.trim() === "") {
      return {
        ...taskData,
        error: "Task description cannot be empty"
      };
    }
    
    try {
      const response = await fetch("http://localhost:3001/api/tasks", {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(taskData)
      });
      
      if (response.ok) {
        // Redirect alla pagina principale dopo l'aggiunta
        navigate('/');
        return initialState;
      } else {
        const errorData = await response.json();
        return {
          ...taskData,
          error: errorData.message || 'Error creating task'
        };
      }
    } catch (error) {
      return {
        ...taskData,
        error: 'Network error, please try again'
      };
    }
  };

  const [state, formAction] = useActionState(handleSubmit, initialState);

  
  return (
    <Container>
      {state.error && <Alert variant="danger">{state.error}</Alert>}
      
      <Form action={formAction} className="task-form">
        <h1 className="form-title">Add New Task</h1>

        {/* Task Description */}
        <Form.Group className="mb-3" controlId="text">
          <Form.Label>Task Description*</Form.Label>
          <Form.Control
            type="text"
            name="text"
            defaultValue={state.text}
            placeholder="Enter task description..."
            required
            minLength={2}
          />
        </Form.Group>

        {/* Priority */}
        <Form.Group className="mb-3" controlId="priority">
          <Form.Label>Priority</Form.Label>
          <Form.Select
            name="priority"
            defaultValue={state.priority}
          >
            <option value="1">Low</option>
            <option value="2">Medium</option>
            <option value="3">High</option>
          </Form.Select>
        </Form.Group>

        {/* Due Date */}
        <Form.Group className="mb-3" controlId="due_date">
          <Form.Label>Due Date</Form.Label>
          <Form.Control
            type="date"
            name="due_date"
            defaultValue={state.due_date}
            min={dayjs().format('YYYY-MM-DD')}
          />
        </Form.Group>

        {/* Completed Checkbox */}
        <Form.Group className="mb-3" controlId="completed">
          <Form.Check
            type="checkbox"
            label="Mark as completed"
            name="completed"
            defaultChecked={state.completed}
          />
        </Form.Group>

        {/* Buttons */}
        <Row className="button-group">
          <Col>
            <Button type="submit" variant="primary" className="me-2">
              Add Task
            </Button>
            <Link to="/" className="btn btn-secondary">
              Cancel
            </Link>
          </Col>
        </Row>
      </Form>
    </Container>
  );
}

export default AddTask;