extern crate redis;
extern crate rustc_serialize;
extern crate time;
use rustc_serialize::json;

pub fn add_user(con: &redis::Connection,
                username: &str,
                firstname: &str,
                lastname: &str,
                email: &str)
                -> Result<(), String> {
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

pub fn create_board(con: &redis::Connection, board: &Board) -> Result<Board, String> {
    let res = match redis::cmd("INCR").arg("id:boards").query(con) {
        Ok(newid) => {
            let board = Board { id: newid, ..board.clone() };
            redis::cmd("SADD").arg("boards").arg(newid).execute(con);
            let encoded = json::encode(&board).unwrap();
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

pub fn add_stickynote(con: &redis::Connection, note: &StickyNote) -> Result<StickyNote, String> {
    let res = match redis::cmd("INCR").arg("id:stickynotes").query(con) {
        Ok(newid) => {
            let ts = get_timestamp();
            let note = StickyNote {
                id: newid,
                timestamp: ts,
                ..note.clone()
            };
            let encoded = json::encode(&note).unwrap();
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

fn get_timestamp() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    let milliseconds = (current_time.sec as i64 * 1000) + (current_time.nsec as i64 / 1000 / 1000);
    milliseconds as u64
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct StickyNote {
    id: u64,
    title: String,
    content: String,
    timestamp: u64,
    owner: String,
    boardid: u64,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Board {
    id: u64,
    name: String,
    owner: String,
    groups: Vec<String>,
}

#[cfg(test)]
mod tests;