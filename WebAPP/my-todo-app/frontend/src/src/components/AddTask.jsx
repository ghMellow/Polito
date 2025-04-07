import { useState } from 'react';
import dayjs from 'dayjs';

function AddTask() {
  const [taskData, setTaskData] = useState({
    text: '',
    priority: 1,
    due_date: dayjs().add(1, 'day').format('YYYY-MM-DD'),
    completed: false
  });
  
  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setTaskData({
      ...taskData,
      [name]: type === 'checkbox' ? checked : value
    });
  };
  
  const handleSubmit = async (e) => {
    e.preventDefault();
    
    try {
      const response = await fetch("http://localhost:3001/api/tasks", {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(taskData)
      });
      
      if (response.ok) {
        // Reset form after successful submission
        setTaskData({
          text: '',
          priority: 1,
          due_date: dayjs().add(1, 'day').format('YYYY-MM-DD'),
          completed: false
        });
        // You might want to trigger a refresh of the task list here
        // or show a success message
      } else {
        const errorData = await response.json();
        console.error('Error creating task:', errorData);
        // Handle error (show message to user, etc.)
      }
    } catch (error) {
      console.error('Error submitting form:', error);
      // Handle network error
    }
  };
  
  const handleReset = () => {
    setTaskData({
      text: '',
      priority: 1,
      due_date: dayjs().add(1, 'day').format('YYYY-MM-DD'),
      completed: false
    });
  };
  
  return (
    <div className="task-form">
      <h1 className="form-title">Add New Task</h1>
      
      <form onSubmit={handleSubmit}>
        {/* Task text input */}
        <div className="form-group">
          <label htmlFor="text" className="form-label">Task Description*</label>
          <input
            id="text"
            name="text"
            type="text"
            value={taskData.text}
            onChange={handleChange}
            className="form-input"
            placeholder="Enter task description..."
            required
          />
        </div>
        
        {/* Priority selection */}
        <div className="form-group">
          <label htmlFor="priority" className="form-label">Priority</label>
          <select
            id="priority"
            name="priority"
            value={taskData.priority}
            onChange={handleChange}
            className="form-select"
          >
            <option value="1">Low</option>
            <option value="2">Medium</option>
            <option value="3">High</option>
          </select>
        </div>
        
        {/* Due date selection */}
        <div className="form-group">
          <label htmlFor="due_date" className="form-label">Due Date</label>
          <input
            id="due_date"
            name="due_date"
            type="date"
            value={taskData.due_date}
            onChange={handleChange}
            className="form-input"
            min={dayjs().format('YYYY-MM-DD')}
          />
        </div>
        
        {/* Completed checkbox */}
        <div className="form-group">
          <div className="checkbox-group">
            <input
              id="completed"
              name="completed"
              type="checkbox"
              checked={taskData.completed}
              onChange={handleChange}
              className="form-checkbox"
            />
            <label htmlFor="completed" className="form-label">Mark as completed</label>
          </div>
        </div>
        
        {/* Form buttons */}
        <div className="button-group">
          <button
            type="submit"
            className="submit-button"
          >
            Add Task
          </button>
          <button
            type="button"
            onClick={handleReset}
            className="reset-button"
          >
            Reset
          </button>
        </div>
      </form>
    </div>
  );
}

export default AddTask;