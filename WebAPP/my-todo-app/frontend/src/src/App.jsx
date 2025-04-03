import './App.css'

import NavHeader from "./components/NavHeader";
import AddTask from "./components/AddTask";
import TasksList from "./components/TasksList";


function App() {
  return (
    <div>
      <NavHeader />
      <AddTask />
      <TasksList />
    </div>
  );
}

export default App
