extern crate redis;
extern crate rustc_serialize;
use rustc_serialize::json;

use {add_user, create_board, Board, add_stickynote, StickyNote};

#[test]
fn add_user_updates_set_and_hash() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    purge_kevin(&con);

    match add_user(&con, "kevin", "Kevin", "Hoffman", "foo@bar.com") {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };

    let s: Vec<String> = redis::cmd("SMEMBERS").arg("users").query(&con).unwrap();
    assert_eq!(1, s.len());
    assert_eq!(&s, &["kevin"]);

    let firstname: String = redis::cmd("HGET")
        .arg("user:kevin")
        .arg("firstname")
        .query(&con)
        .unwrap();
    assert_eq!(firstname, "Kevin");

    let lastname: String = redis::cmd("HGET")
        .arg("user:kevin")
        .arg("lastname")
        .query(&con)
        .unwrap();
    assert_eq!(lastname, "Hoffman");

    let email: String = redis::cmd("HGET")
        .arg("user:kevin")
        .arg("email")
        .query(&con)
        .unwrap();
    assert_eq!(email, "foo@bar.com");

    purge_kevin(&con);

}

#[test]
fn create_board_creates_appropriate_structures() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    purge_boards(&con);

    redis::cmd("INCRBY").arg("id:boards").arg(89).execute(&con);
    let board = Board {
        id: 0,
        name: "Test board".to_string(),
        owner: "kevin".to_string(),
        groups: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    match create_board(&con, &board) {
        Ok(board) => {
            let ids: Vec<u64> = redis::cmd("SMEMBERS").arg("boards").query(&con).unwrap();
            assert_eq!(1, ids.len());
            let s: String = redis::cmd("GET").arg("board:90").query(&con).unwrap();
            let decoded: Board = json::decode(&s).unwrap();
            assert_eq!(decoded.id, 90);
            assert_eq!(board.id, decoded.id);
            assert_eq!(decoded.name, "Test board");
            assert_eq!(decoded.owner, "kevin");
            assert_eq!(&decoded.groups, &["a", "b", "c"]);
        }
        Err(_) => assert!(false),
    };

    purge_boards(&con);
}

#[test]
fn add_stickynote_creates_appropriate_structures() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    purge_notes(&con);

    redis::cmd("INCRBY").arg("id:stickynotes").arg(89).execute(&con);
    let note = StickyNote {
        id: 0,
        timestamp: 0,
        title: "New Note".to_string(),
        content: "Content".to_string(),
        owner: "kevin".to_string(),
        boardid: 325,
    };
    match add_stickynote(&con, &note) {
        Ok(note) => {
            let ids: Vec<u64> = redis::cmd("ZRANGE")
                .arg("board:325:stickynotes")
                .arg(0)
                .arg(-1)
                .query(&con)
                .unwrap();
            assert_eq!(1, ids.len());
            let s: String = redis::cmd("GET").arg("stickynote:90").query(&con).unwrap();
            let decoded: StickyNote = json::decode(&s).unwrap();
            assert_eq!(90, decoded.id);
            assert_eq!(note.id, decoded.id);
            assert_eq!("New Note", decoded.title);
            assert_eq!("Content", decoded.content);
        }
        Err(_) => assert!(false),
    };

    purge_notes(&con);
}

fn purge_notes(con: &redis::Connection) {
    redis::cmd("DEL").arg("stickynote:90").execute(con);
    redis::cmd("DEL").arg("board:325:stickynotes").execute(con);
    redis::cmd("DEL").arg("id:stickynotes").execute(con);
}

fn purge_boards(con: &redis::Connection) {
    redis::cmd("DEL").arg("boards").execute(con);
    redis::cmd("DEL").arg("id:boards").execute(con);
    redis::cmd("DEL").arg("board:90").execute(con);
}

fn purge_kevin(con: &redis::Connection) {
    redis::cmd("DEL").arg("users").execute(con);
    redis::cmd("HDEL")
        .arg("user:kevin")
        .arg("firstname")
        .execute(con);
    redis::cmd("HDEL")
        .arg("user:kevin")
        .arg("lastname")
        .execute(con);
    redis::cmd("HDEL")
        .arg("user:kevin")
        .arg("email")
        .execute(con);
}