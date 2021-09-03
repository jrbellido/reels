use sqlite::Connection;
use sqlite::State;

use crate::reference::{Reference, ReferenceType};
use crate::snapshot::Snapshot;

pub trait Database {
    fn store_snapshot(&self, snapshot: &Snapshot);

    fn find_snapshot(&self, id: &str) -> Snapshot;

    fn start_transaction(&self);

    fn end_transaction(&self);

    fn store_head(&self, reference: &Reference, is_file: i64, first_chunk: Option<String>);

    fn store_reference(&self, reference: &Reference, snapshot_id: &str);

    fn reference_exists(&self, reference_id: &str) -> bool;

    fn find_snapshots(&self) -> Vec<Snapshot>;

    fn find_references(&self, snapshot: &Snapshot) -> Vec<Reference>;

    fn remove_snapshot(&self, snapshot: &Snapshot);
}

pub struct DatabaseSqlite {
    con: Connection,
}

impl DatabaseSqlite {
    pub fn open(path: &str) -> Self {
        let con = sqlite::open(path).unwrap();
        let _r = create_schema(&con);
        Self { con }
    }
}

impl Database for DatabaseSqlite {
    fn store_snapshot(&self, snapshot: &Snapshot) {
        self.con
            .execute(format!(
                "INSERT INTO snapshots (id, created_on) VALUES ('{}', '{}');",
                snapshot.id, snapshot.created_on
            ))
            .unwrap_or_else(|err| eprintln!("Error writing snapshot: {}", err));
    }

    fn find_snapshot(&self, id: &str) -> Snapshot {
        let mut tx = self
            .con
            .prepare(
                "SELECT id, created_on
                 FROM snapshots
                 WHERE id LIKE :reference_id || '%';",
            )
            .unwrap();
        tx.bind_by_name(":reference_id", id).unwrap();
        tx.next().unwrap();
        Snapshot::from_database(tx.read::<String>(0).unwrap(), tx.read::<String>(1).unwrap())
    }

    fn start_transaction(&self) {
        self.con
            .execute("BEGIN TRANSACTION;")
            .unwrap_or_else(|err| eprintln!("Error beginning transaction: {}", err));
    }

    fn end_transaction(&self) {
        self.con
            .execute("END TRANSACTION;")
            .unwrap_or_else(|err| eprintln!("Error closing transaction: {}", err));
    }

    fn store_head(&self, reference: &Reference, is_file: i64, first_chunk: Option<String>) {
        let mut tx = self
            .con
            .prepare(
                "INSERT INTO head (reference_id, path, original_path, is_file, first_chunk)
                     VALUES (:reference_id, :path, :original_path, :is_file, :first_chunk);",
            )
            .unwrap();

        tx.bind_by_name(":reference_id", reference.id.as_str())
            .unwrap();
        tx.bind_by_name(":path", reference.path.as_str()).unwrap();
        tx.bind_by_name(":original_path", reference.original_path.as_str())
            .unwrap();
        tx.bind_by_name(":is_file", is_file).unwrap();
        tx.bind_by_name(":first_chunk", first_chunk.as_ref().map(String::as_str))
            .unwrap();
        tx.next().unwrap();
    }

    fn store_reference(&self, reference: &Reference, snapshot_id: &str) {
        let mut tx = self
            .con
            .prepare(
                "INSERT INTO snapshot_refs (reference_id, snapshot_id)
                       VALUES (:reference_id, :snapshot_id);",
            )
            .unwrap();

        tx.bind_by_name(":reference_id", reference.id.as_str())
            .unwrap();
        tx.bind_by_name(":snapshot_id", snapshot_id).unwrap();
        tx.next().unwrap();
    }

    fn reference_exists(&self, reference_id: &str) -> bool {
        let mut tx = self
            .con
            .prepare("SELECT count(1) FROM head WHERE reference_id = :reference_id;")
            .unwrap();

        tx.bind_by_name(":reference_id", reference_id).unwrap();
        tx.next().unwrap();
        tx.read::<i64>(0).unwrap() > 0
    }

    fn find_snapshots(&self) -> Vec<Snapshot> {
        let mut cursor = self
            .con
            .prepare("SELECT id, created_on FROM snapshots;")
            .unwrap()
            .into_cursor();

        let mut result: Vec<Snapshot> = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let snapshot_id = abbreviate_id(row[0].as_string().unwrap());
            let created_on = row[1].as_string().unwrap().to_string();
            let snapshot = Snapshot::from_database(snapshot_id, created_on);
            result.push(snapshot);
        }

        return result;
    }

    fn find_references(&self, snapshot: &Snapshot) -> Vec<Reference> {
        let mut tx = self
            .con
            .prepare(format!(
                "SELECT h.reference_id, h.is_file, h.path, h.original_path, h.first_chunk
             FROM snapshot_refs sr
             LEFT JOIN head h ON h.reference_id = sr.reference_id
             WHERE sr.snapshot_id LIKE '{}%';",
                &snapshot.id
            ))
            .unwrap();

        let mut result: Vec<Reference> = Vec::new();

        while let State::Row = tx.next().unwrap() {
            let id = tx.read::<String>(0).unwrap().to_string();
            let reference_type = match tx.read::<i64>(1).unwrap() == 1 {
                true => ReferenceType::File,
                false => ReferenceType::Directory,
            };
            let path = tx.read::<String>(2).unwrap().to_string();
            let original_path = tx.read::<String>(3).unwrap().to_string();
            let first_chunk = match tx.read::<String>(4) {
                Ok(s) => s.to_string(),
                _ => "".to_string(),
            };
            let first_chunk_id = match first_chunk.len() > 0 {
                true => Some(first_chunk),
                false => None,
            };
            let snapshot_ref =
                Reference::from(id, reference_type, path, original_path, first_chunk_id);
            result.push(snapshot_ref);
        }

        return result;
    }

    fn remove_snapshot(&self, snapshot: &Snapshot) {
        self.con
            .execute(format!(
                "DELETE FROM snapshots WHERE id = '{}'",
                snapshot.id
            ))
            .unwrap();

        self.con
            .execute(format!(
                "DELETE FROM snapshot_refs WHERE snapshot_id = '{}'",
                snapshot.id
            ))
            .unwrap();
    }
}

fn create_schema(con: &Connection) -> sqlite::Result<()> {
    con.execute(
        "CREATE TABLE head (
           reference_id TEXT PRIMARY KEY, 
           is_file INTEGER, 
           path TEXT, 
           original_path TEXT,
           first_chunk TEXT);",
    )?;

    con.execute(
        "CREATE TABLE snapshots (
           id TEXT PRIMARY KEY, 
           created_on TEXT);",
    )?;

    con.execute(
        "CREATE TABLE snapshot_refs (
           reference_id TEXT, 
           snapshot_id TEXT);",
    )?;

    Ok(())
}

fn abbreviate_id(id: &str) -> String {
    id.chars().into_iter().take(12).collect()
}
