// AddTask.jsx
import { useState } from 'react';

function AddTask() {
  const [task, setTask] = useState('');
  const [width, setWidth] = useState(500);
  const [height, setHeight] = useState(33);
  
  const handleSubmit = (e) => {
    e.preventDefault();
    // Add task handling logic here
    setTask('');
  };
  
  const handleReset = () => {
    setTask('');
  };
  
  return (
    <div style={{ display: "flex", alignItems: "center" }}>
      <form onSubmit={handleSubmit} className="flex flex-wrap gap-2 items-center">
        <input
            type="text"
            value={task}
            onChange={(e) => setTask(e.target.value)}
            style={{ width: `${width}px`, height: `${height}px` }}
            className="p-3 border border-gray-300 rounded resize-x"
            placeholder="Add your item here..."
        />
        <button 
            type="submit"
            onClick={handleSubmit}
            style={{ marginLeft: "8px" }}
            className="whitespace-nowrap px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
          >
            Submit
          </button>
          <button
            type="button"
            onClick={handleReset}
            style={{ marginLeft: "8px" }}
            className="whitespace-nowrap px-4 py-2 border border-red-500 text-red-500 rounded hover:bg-red-50"
          >
            Reset
          </button>
      </form>
    </div>
  );
}

export default AddTask;