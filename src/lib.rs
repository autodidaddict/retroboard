extern crate redis;
extern crate rustc_serialize;
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

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Board {
    id: u64,
    name: String,
    owner: String,
    groups: Vec<String>,
}

#[cfg(test)]
mod tests;