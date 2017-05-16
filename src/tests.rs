extern crate redis;
extern crate rustc_serialize;
use rustc_serialize::json;

use add_user;

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