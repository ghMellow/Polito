use std::collections::HashMap;

struct Albero{
    // mappe la key deve essere univoca
    children_map: HashMap<String, Vec<String>>,  // per le relazioni padre → figli.
    parent_map: HashMap<String, String>,        // per tenere traccia del padre di ciascun nodo child → parent (utile per risalire alla radice).
    switches: HashMap<String, bool>             // per lo stato dell’interruttore dei nodi.
}

impl Albero {
    // nota: aggiustare mutabilità dove necessario gestire errori in caso
    // di collisioni, valori mancanti
    // aggiungi un nodo figlio del nodo father
    pub fn add(&self, father: &str, node: &str) {unimplemented!()}
    // togli un nodo e tutti gli eventuali rami collegati
    pub fn remove(&self, node: &str) {unimplemented!()}
    // commuta l’interruttore del nodo (che può essere on off) e restituisci il nuovo valore
    pub fn toggle(&self, node: &str) -> bool {unimplemented!()}
    // restituisci se la luce è accesa e spenta
    pub fn peek(&self, node: &str) -> bool {unimplemented!()}
}

