use crossbeam_channel as channel;
use crossbeam_channel::{Receiver, Sender, select};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Tipo per i dati dei sensori
type SensorData = (String, i32);

fn sensor_producer(sensor_id: String, tx: Sender<SensorData>) {
    for _ in 1..=3 {
        let value = rand::random_range(0..100);
        tx.send((sensor_id.clone(), value)).unwrap();
        // Riduci il tempo di sleep dei sensori
        thread::sleep(Duration::from_millis(rand::random_range(100..400)));
    }
}

fn command_producer(tx: Sender<String>) {
    let commands = vec![
        "print sensor1".to_string(),
        "print sensor2".to_string(),
        "print sensor3".to_string(),
    ];
    for cmd in commands {
        // Aumenta il tempo di sleep tra i comandi
        thread::sleep(Duration::from_secs(rand::random_range(4..7)));
        tx.send(cmd).unwrap();
    }
}

fn monitor(
    rx_sensor: Receiver<SensorData>,
    rx_cmd: Receiver<String>,
    n: usize,
) {
    let mut data: HashMap<String, Vec<i32>> = HashMap::new();
    let mut sensor_closed = false;
    let mut cmd_closed = false;
    loop {
        if sensor_closed && cmd_closed {
            break;
        }
        select! {
            recv(rx_sensor) -> msg => {
                match msg {
                    Ok((id, value)) => {
                        let entry = data.entry(id).or_insert_with(Vec::new);
                        entry.push(value);
                        if entry.len() > n {
                            entry.remove(0);
                        }
                    }
                    Err(_) => sensor_closed = true,
                }
            }
            recv(rx_cmd) -> msg => {
                match msg {
                    Ok(cmd) => {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        if parts.len() == 2 && parts[0] == "print" {
                            let sensor_id = parts[1];
                            if let Some(values) = data.get(sensor_id) {
                                println!("Ultimi valori di {}: {:?}", sensor_id, values);
                            } else {
                                println!("Nessun dato per {}", sensor_id);
                            }
                        }
                    }
                    Err(_) => cmd_closed = true,
                }
            }
        }
    }
}

fn main() {
    let (tx_sensor, rx_sensor) = channel::unbounded();
    let (tx_cmd, rx_cmd) = channel::unbounded();

    // Avvia 3 sensori
    let mut handles = vec![];
    for i in 1..=3 {
        let tx = tx_sensor.clone();
        let handle = thread::spawn(move || {
            sensor_producer(format!("sensor{}", i), tx);
        });
        handles.push(handle);
    }

    // Avvia la console dei comandi
    let tx_cmd_clone = tx_cmd.clone();
    let cmd_handle = thread::spawn(move || {
        command_producer(tx_cmd_clone);
    });

    // Chiudi tx_sensor quando tutti i sensori hanno finito
    for handle in handles {
        handle.join().unwrap();
    }
    drop(tx_sensor);

    // Chiudi tx_cmd quando la console ha finito
    cmd_handle.join().unwrap();
    drop(tx_cmd);

    // Thread di monitoraggio
    monitor(rx_sensor, rx_cmd, 5);
}