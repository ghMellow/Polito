import './App.css'

import NavHeader from "./components/NavHeader";
import AddTask from "./components/AddTask";
import TasksList from "./components/TasksList";


function App() {
  return (
  <div className="w-full min-h-screen">
      <NavHeader />
      <AddTask />
      <br /><br />
      <TasksList />
    </div>
  );
}

export default App
