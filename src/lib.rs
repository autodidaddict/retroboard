extern crate redis;
extern crate serde;
extern crate serde_json;
extern crate time;

#[macro_use]
extern crate serde_derive;

pub struct Retroboard {
    client: redis::Client,
}

impl Retroboard {
    pub fn new(cs: &str) -> Self {
        Retroboard { client: redis::Client::open(cs).unwrap() }
    }


    pub fn add_user(&self,
                    username: &str,
                    firstname: &str,
                    lastname: &str,
                    email: &str)
                    -> Result<(), String> {
        let con = &self.client.get_connection().unwrap();
        let key = format!("user:{}", username);
        redis::cmd("SADD")
            .arg("users")
            .arg(username)
            .execute(con);
        redis::cmd("HSET")
            .arg(&key)
            .arg("firstname")
            .arg(firstname)
            .execute(con);
        redis::cmd("HSET")
            .arg(&key)
            .arg("lastname")
            .arg(lastname)
            .execute(con);
        redis::cmd("HSET")
            .arg(&key)
            .arg("email")
            .arg(email)
            .execute(con);
        Ok(())
    }

    pub fn create_board(&self, board: &Board) -> Result<Board, String> {
        let con = &self.client.get_connection().unwrap();
        let res = match redis::cmd("INCR").arg("id:boards").query(con) {
            Ok(newid) => {
                let board = Board { id: newid, ..board.clone() };
                redis::cmd("SADD").arg("boards").arg(newid).execute(con);
                let encoded = serde_json::to_string(&board).unwrap();
                redis::cmd("SET")
                    .arg(format!("board:{}", board.id))
                    .arg(encoded)
                    .execute(con);
                Ok(board)
            }
            Err(e) => Err(format!("{}", e)),
        };
        res
    }

    pub fn get_boards(&self) -> Result<Vec<Board>, String> {
        let con = &self.client.get_connection().unwrap();
        let mut boards: Vec<Board> = Vec::new();

        let ids: Vec<u64> = redis::cmd("SMEMBERS").arg("boards").query(con).unwrap();
        for id in ids {
            let s: String = redis::cmd("GET").arg(format!("board:{}", id)).query(con).unwrap();
            let decoded: Board = serde_json::from_str(&s).unwrap();
            boards.push(decoded);
        }
        Ok(boards)
    }

    pub fn add_stickynote(&self, note: &StickyNote) -> Result<StickyNote, String> {
        let con = &self.client.get_connection().unwrap();
        let res = match redis::cmd("INCR").arg("id:stickynotes").query(con) {
            Ok(newid) => {
                let ts = get_timestamp();
                let note = StickyNote {
                    id: newid,
                    timestamp: ts,
                    ..note.clone()
                };
                let encoded = serde_json::to_string(&note).unwrap();
                // zadd board:1:stickynotes {stamp} {id}
                redis::cmd("ZADD")
                    .arg(format!("board:{}:stickynotes", note.boardid))
                    .arg(ts)
                    .arg(newid)
                    .execute(con);
                redis::cmd("SET")
                    .arg(format!("stickynote:{}", note.id))
                    .arg(encoded)
                    .execute(con);
                Ok(note)
            }
            Err(e) => Err(format!("{}", e)),
        };
        res
    }


    pub fn get_stickynotes(&self, board_id: u64) -> Result<Vec<StickyNote>, String> {
        let con = &self.client.get_connection().unwrap();
        let mut notes: Vec<StickyNote> = Vec::new();

        let ids: Vec<u64> = redis::cmd("SMEMBERS")
            .arg(format!("board:{}:stickynotes", board_id))
            .query(con)
            .unwrap();
        for id in ids {
            let s: String = redis::cmd("GET").arg(format!("stickynote:{}", id)).query(con).unwrap();
            let decoded: StickyNote = serde_json::from_str(&s).unwrap();
            notes.push(decoded);
        }
        Ok(notes)
    }
}

fn get_timestamp() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    let milliseconds = (current_time.sec as i64 * 1000) + (current_time.nsec as i64 / 1000 / 1000);
    milliseconds as u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StickyNote {
    id: u64,
    title: String,
    content: String,
    timestamp: u64,
    owner: String,
    boardid: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    id: u64,
    name: String,
    owner: String,
    groups: Vec<String>,
}

#[cfg(test)]
mod tests;
