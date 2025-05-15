use std::collections::HashMap;

pub struct Albero{
    // mappe la key deve essere univoca
    pub children_map: HashMap<String, Vec<String>>,  // per le relazioni padre → figli.
    pub parent_map: HashMap<String, String>,         // per tenere traccia del padre di ciascun nodo child → parent (utile per risalire alla radice).
    pub switches: HashMap<String, bool>              // per lo stato dell’interruttore dei nodi.
}

impl Albero {
    // nota: aggiustare mutabilità dove necessario gestire errori in caso
    // di collisioni, valori mancanti

    // inizializzazione dell'albero
    pub fn new() -> Self {
        let mut tree = Albero{ children_map: HashMap::new(), parent_map: HashMap::new(), switches: HashMap::new() };
        tree.children_map.insert(String::from("Root"), Vec::new());
        tree.switches.insert(String::from("Root"), true);

        tree
    }

    // aggiungi un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) {
        if self.check_path(father) {
            if let Some(children) = self.children_map.get_mut(&father.to_string()) {
                children.push(node.to_string()); // aggiungo figlio
            } else {
                self.children_map.insert(father.to_string(), vec![node.to_string()]); // creo padre e aggiungo figlio
            }

            self.parent_map.insert(node.to_string(), father.to_string()); // aggiungo child → parent
            self.switches.insert(node.to_string(), false); // setto lo switch
        } else {
            println!("Invalid father '{father}', path have no connection to the Root");
        }
    }

    // togli un nodo e tutti gli eventuali rami collegati
    pub fn remove(&mut self, node: &str) {
        // children conterrà il clone dei figli 'kids'
        let children = match self.children_map.get(node) {
            Some(kids) => kids.clone(), // Cloniamo per evitare problemi di borrow
            None => Vec::new() // vet vuoto così da avere un uscita semplice nel for ricorsivo
        };

        // Rimuoviamo ricorsivamente ogni figlio
        for child in children {
            self.remove(&child);
        }

        // Nodo senza figli,
        // Prima elimino la presenza residua del nodo dal padre
        // infatti la funzione ricorsiva guarda i figli del nodo no padre
        if let Some(father) = self.parent_map.get(node) {
            let childrens = self.children_map.get_mut(father).unwrap();

            /* Scam di rust borrow checker non permette di modificare qualcosa mentre ci si cicla
              for (index, child) in childrens.iter().enumerate() {
                if child == node {
                    childrens.remove(index);
                }
              }
            */
            // Find the index first, then remove it
            if let Some(index) = childrens.iter().position(|child| child == node) {
                childrens.remove(index);
            }
        }

        // infine rimuovo nodo da mappe
        self.children_map.remove(node); // qua poiché deve essere un padre per essere presente nella mappa
        self.parent_map.remove(node);
        self.switches.remove(node);
    }

    // commuta l’interruttore del nodo (che può essere on off) e restituisci il nuovo valore
    pub fn toggle(&mut self, node: &str) -> Option<bool> {
        if node == "Root" {
            None
        } else {
            if let Some(switch) = self.switches.get_mut(node) {
                if *switch {
                    self.switches.insert(node.to_string(), false); // sovrascrivo valore
                    Some(false)
                } else {
                    self.switches.insert(node.to_string(), true);
                    Some(true)
                }
            } else { None } // if let obbliga a gestire il caso di ritorno negativo nell'else
        }
    }

    // controlla se il nodo appartiene a un percorso che arriva alla radice
    pub fn check_path(&self, node: &str) -> bool {
        let mut current = node;
        while let Some(parent) = self.parent_map.get(current) {
            current = parent;
        }

        current == "Root"
    }

    // restituisci se la luce è accesa e spenta
    pub fn peek(&self, node: &str) -> Option<bool> {
        // Verifico prima di tutto se il nodo esiste
        let switch_state = self.switches.get(node)?; // ritorna None se non trova il nodo

        // Se l'interruttore del nodo è spento, ritorna false subito
        if !*switch_state {
            return Some(false);
        }

        // Altrimenti, risali l'albero fino alla radice controllando
        // che tutti gli interruttori dei nodi genitori siano attivi
        let mut current = node;

        while let Some(parent) = self.parent_map.get(current) {
            // Controlla lo stato dell'interruttore del genitore
            if let Some(parent_switch) = self.switches.get(parent) {
                if !*parent_switch {
                    return Some(false);
                }
            } else {
                // Genitore senza switch, situazione inaspettata
                return None;
            }

            // Se siamo arrivati alla radice, interrompi il ciclo
            if parent == "Root" {
                break;
            }

            // Passa al genitore per la prossima iterazione
            current = parent;
        }

        // Se arriviamo qui, tutti gli interruttori nel percorso fino alla radice sono attivi
        Some(true)
    }
}

