import { useActionState } from "react";
import { Container, Form, Button, Row, Col, Alert } from 'react-bootstrap';
import { useNavigate, useParams, Link } from 'react-router-dom';
import dayjs from 'dayjs';
import { useTasks } from "./TaskContext";

export function EditTaskForm() {
  const { id } = useParams();
  const { tasks, setTasks } = useTasks();
  const task = tasks.find(t => t.id === parseInt(id));

  console.log("Tasks from context:", tasks);
  console.log("Task ID from URL:", id);
  console.log("Task found:", task);


  if (task) {
    return <TaskForm task={task} setTasks={setTasks} />;
  } else {
    return (
      <Container>
        <Alert variant="danger">
          Impossibile modificare un task non esistente! <Link to="/">Torna alla lista</Link>
        </Alert>
      </Container>
    );
  }
}

export function TaskForm({ task, setTasks }) {
  const navigate = useNavigate();

  // Corretto il formato e l'utilizzo dei campi per allinearsi con il backend
  const initialState = {
    text: task?.text || '',
    priority: task?.priority || '1',
    dueDate: task?.dueDate ? dayjs(task.dueDate).format('YYYY-MM-DD') : dayjs().add(1, 'day').format('YYYY-MM-DD'),
    completed: task?.completed || false,
    error: null
  };

  const handleSubmit = async (prevState, formData) => {
    const taskData = Object.fromEntries(formData.entries());
    taskData.completed = formData.has('completed');

    if (taskData.text.trim() === "") {
      return { ...taskData, error: "Task description cannot be empty" };
    }

    try {
      const response = await fetch(`http://localhost:3001/api/tasks/${task.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(taskData)
      });

      if (response.ok) {
        const updatedTask = { id: task.id, ...taskData };
        setTasks(prev => prev.map(t => t.id === task.id ? updatedTask : t));
        navigate('/');
        return initialState;
      } else {
        const errorData = await response.json();
        return { ...taskData, error: errorData.message || "Update error" };
      }
    } catch (error) {
      return { ...taskData, error: "Network error, please try again" };
    }
  };

  const [state, formAction] = useActionState(handleSubmit, initialState);

  return (
    <Container>
      {state.error && <Alert variant="danger">{state.error}</Alert>}
      <Form action={formAction} className="task-form">
        <h1 className="form-title">Edit Task</h1>

        <Form.Group className="mb-3" controlId="text">
          <Form.Label>Task Description*</Form.Label>
          <Form.Control
            type="text"
            name="text"
            defaultValue={state.text}
            required
            minLength={2}
          />
        </Form.Group>

        <Form.Group className="mb-3" controlId="priority">
          <Form.Label>Priority</Form.Label>
          <Form.Select name="priority" defaultValue={state.priority}>
            <option value="1">Low</option>
            <option value="2">Medium</option>
            <option value="3">High</option>
          </Form.Select>
        </Form.Group>

        <Form.Group className="mb-3" controlId="dueDate">
          <Form.Label>Due Date</Form.Label>
          <Form.Control
            type="date"
            name="dueDate"
            defaultValue={state.dueDate}
            min={dayjs().format('YYYY-MM-DD')}
          />
        </Form.Group>

        <Form.Group className="mb-3" controlId="completed">
          <Form.Check
            type="checkbox"
            label="Mark as completed"
            name="completed"
            defaultChecked={state.completed}
          />
        </Form.Group>

        <Row className="button-group">
          <Col>
            <Button type="submit" variant="success" className="me-2">Update Task</Button>
            <Link to="/" className="btn btn-secondary">Cancel</Link>
          </Col>
        </Row>
      </Form>
    </Container>
  );
}

export default EditTaskForm;