pub mod filesystem {

    struct  Directory {}
    struct  File {}
    struct  Link {}

    enum FSItem {
        Directory(Directory), // Directory contiene nome, i figli, eventuali metadati, il padre
        File(File), // File contiene il nome, eventuali metadati (es dimensione, owner, ecc), il padre
        SymLink(Link) // Il link simbolico contiene il Path a cui punta e il padre
    }

    struct FileSystem{
    }

    impl FileSystem {
        // crea un nuovo FS vuoto
        //pub fn new() -> Self

        // crea un nuovo FS replicando la struttura su disco
        //pub fn from_disk() -> Self

        // cambia la directory corrente, path come in tutti gli altri metodi
        // può essere assoluto o relativo;
        // es: “../sibling” vuol dire torna su di uno e scendi in sibling
        //pub fn change_dir(&mut self, path: String) -> Result

        // crea la dir in memoria e su disco
        //pub fn make_dir(&self, path: String, name: String) -> Result

        // crea un file vuoto in memoria e su disco
        //pub fn make_dir(&self, path: String, name: String) -> Result

        // rinomina file / dir in memoria e su disco
        //pub fn rename(&self, path: String, new_name: String) -> Result

        // cancella file / dir in memoria e su disco, se è una dir cancella tutto il contenuto
        //pub fn delete(&self, path: String) -> Result

        // cerca l’elemento indicato dal path e restituisci un riferimento
        //pub find(&self, path: String) -> Result
    }
}