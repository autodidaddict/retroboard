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

#[cfg(test)]
mod tests;