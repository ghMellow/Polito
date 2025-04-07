// test-api.js - Script per testare gli endpoint dell'API
import fetch from 'node-fetch';
import dayjs from 'dayjs';

const API_URL = 'http://localhost:3001/api';
let createdTaskId = null;

// Funzione per formattare il risultato JSON
const formatJSON = (obj) => JSON.stringify(obj, null, 2);

// Test per ottenere tutti i task
async function getAllTasks() {
  console.log('\n--- TEST: Ottieni tutti i task ---');
  
  try {
    const response = await fetch(`${API_URL}/tasks`);
    const tasks = await response.json();
    
    console.log(`Status: ${response.status}`);
    console.log('Tasks:', formatJSON(tasks));
    
    return tasks;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Test per creare un nuovo task
async function createTask() {
  console.log('\n--- TEST: Crea un nuovo task ---');
  
  const newTask = {
    text: 'Task di test creato via API',
    priority: 2,
    due_date: dayjs().add(1, 'day').format('YYYY-MM-DD') // Domani con dayjs
  };
  
  
  try {
    const response = await fetch(`${API_URL}/tasks`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(newTask)
    });
    
    const task = await response.json();
    
    console.log(`Status: ${response.status}`);
    console.log('Task creato:', formatJSON(task));
    
    createdTaskId = task.id;
    return task;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Test per ottenere un task specifico
async function getTask(id) {
  console.log(`\n--- TEST: Ottieni task con ID ${id} ---`);
  
  try {
    const response = await fetch(`${API_URL}/tasks/${id}`);
    const task = await response.json();
    
    console.log(`Status: ${response.status}`);
    console.log('Task:', formatJSON(task));
    
    return task;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Test per aggiornare un task
async function updateTask(id) {
  console.log(`\n--- TEST: Aggiorna task con ID ${id} ---`);
  
  const updatedTask = {
    text: 'Task aggiornato via API',
    priority: 3
  };
  
  try {
    const response = await fetch(`${API_URL}/tasks/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(updatedTask)
    });
    
    const task = await response.json();
    
    console.log(`Status: ${response.status}`);
    console.log('Task aggiornato:', formatJSON(task));
    
    return task;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Test per invertire lo stato di completamento
async function toggleTask(id) {
  console.log(`\n--- TEST: Inverti stato completamento task con ID ${id} ---`);
  
  try {
    const response = await fetch(`${API_URL}/tasks/${id}/toggle`, {
      method: 'PATCH'
    });
    
    const task = await response.json();
    
    console.log(`Status: ${response.status}`);
    console.log('Task aggiornato:', formatJSON(task));
    
    return task;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Test per eliminare un task
async function deleteTask(id) {
  console.log(`\n--- TEST: Elimina task con ID ${id} ---`);
  
  try {
    const response = await fetch(`${API_URL}/tasks/${id}`, {
      method: 'DELETE'
    });
    
    console.log(`Status: ${response.status}`);
    console.log(response.status === 204 ? 'Task eliminato con successo' : 'Errore nell\'eliminazione');
    
    return response.status === 204;
  } catch (error) {
    console.error('Errore:', error.message);
  }
}

// Esegui tutti i test in sequenza
async function runAllTests() {
  console.log('=== INIZIO DEI TEST DELL\'API ===');
  
  // Prima vediamo i task esistenti
  await getAllTasks();
  
  // Crea un nuovo task
  const newTask = await createTask();
  
  if (newTask && newTask.id) {
    // Ottieni il task appena creato
    await getTask(newTask.id);
    
    // Aggiorna il task
    await updateTask(newTask.id);
    
    // Inverti lo stato di completamento
    await toggleTask(newTask.id);
    
    // Elimina il task (decommentare per testare l'eliminazione)
    await deleteTask(newTask.id);
  }
  
  // Verifica finale: lista aggiornata di task
  await getAllTasks();
  
  console.log('\n=== FINE DEI TEST ===');
}

// Esegui i test
runAllTests().catch(console.error);