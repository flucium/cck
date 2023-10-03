use std::path::Path;

use rusqlite as sqlite;

use crate::{sql,User};

// use crate::{sql, Key, PrivateKey, PublicKey, User};

pub struct RingBuilder(Ring);

impl RingBuilder {
    pub fn new(path: &Path) -> cck_common::Result<Self> {
        let mut ring = Ring::open(path)?;

        init_tables(&mut ring.0)?;

        Ok(Self(ring))
    }

    pub fn new_in_memory() -> cck_common::Result<Self> {
        let mut ring = Ring::open_in_memory()?;

        init_tables(&mut ring.0)?;

        Ok(Self(ring))
    }

    pub fn build(self) -> Ring {
        self.0
    }
}

pub struct Ring(sqlite::Connection);

impl Ring {
    /// Open database.
    pub fn open(path: &Path) -> cck_common::Result<Self> {
        Ok(Self(
            sqlite::Connection::open(path).map_err(|_| cck_common::Error)?,
        ))
    }

    /// Open database in memory.
    pub fn open_in_memory() -> cck_common::Result<Self> {
        Ok(Self(
            sqlite::Connection::open_in_memory().map_err(|_| cck_common::Error)?,
        ))
    }

    /// Close database
    ///
    /// # Errors
    /// If the database is not closed, it will return an error.
    pub fn close(self) -> cck_common::Result<()> {
        self.0.cache_flush().map_err(|_| cck_common::Error)?;
        self.0.close().map_err(|_| cck_common::Error)
    }

    fn insert_user(&mut self, user: impl Into<User>) -> cck_common::Result<()> {
        let user = user.into();
        self.0
            .execute(
                sql::SQL_INSERT_INTO_USERS,
                sqlite::params![user.id(), user.name(), user.email()],
            )
            .map_err(|_| cck_common::Error)?;

        Ok(())
    }

    
}

fn init_tables(conn: &mut sqlite::Connection) -> cck_common::Result<()> {
    // Create table `users` if not exists.
    conn.execute_batch(sql::SQL_CREATE_TABLE_USERS)
        .map_err(|_| cck_common::Error)?;

    // Create table `private_keys` if not exists.
    conn.execute_batch(sql::SQL_CREATE_TABLE_PRIVATE_KEYS)
        .map_err(|_| cck_common::Error)?;

    // Create table `public_keys` if not exists.
    conn.execute_batch(sql::SQL_CREATE_TABLE_PUBLIC_KEYS)
        .map_err(|_| cck_common::Error)?;

    Ok(())
}

fn drop_tables(conn: &sqlite::Connection) -> cck_common::Result<()> {
    conn.execute(sql::SQL_DROP_TABLE_USERS, [])
        .map_err(|_| cck_common::Error)?;

    conn.execute(sql::SQL_DROP_TABLE_PRIVATE_KEYS, [])
        .map_err(|_| cck_common::Error)?;

    conn.execute(sql::SQL_DROP_TABLE_PUBLIC_KEYS, [])
        .map_err(|_| cck_common::Error)?;

    Ok(())
}

// fn remove_database_file(path: &Path) -> cck_common::Result<()> {
//     if path.exists() {
//         std::fs::remove_file(path).map_err(|_| cck_common::Error)
//     } else if path.is_dir() {
//         Err(cck_common::Error)
//     } else {
//         Err(cck_common::Error)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn ring_build() {
        let ring = RingBuilder::new_in_memory().unwrap().build();

        assert_eq!(ring.0.path().unwrap(), String::default());
    }

    #[test]
    fn ring_insert_user() {
        let mut ring = RingBuilder::new_in_memory().unwrap().build();

        let user = User {
            id: "id".to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
        };

        ring.insert_user(&user).unwrap();

        let user = ring.get_user_where_id("id").unwrap();

        assert_eq!(user.id, "id");
        assert_eq!(user.name, "name");
        assert_eq!(user.email, "email");
    }

    #[test]
    fn ring_get_user_where_id() {
        let mut ring = RingBuilder::new_in_memory().unwrap().build();

        let user = User::new("example", "example@example.com").unwrap();

        ring.insert_user(&user).unwrap();

        let get_user = ring.get_user_where_id(user.id()).unwrap();

        assert_eq!(get_user, user);
    }

    #[test]
    fn ring_get_user_where_email() {
        let mut ring = RingBuilder::new_in_memory().unwrap().build();

        let user = User::new("example", "example@example.com").unwrap();

        ring.insert_user(&user).unwrap();

        let get_user = ring.get_user_where_email("example@example.com").unwrap();

        assert_eq!(get_user, user);
    }

    #[test]
    fn ring_get_user_where_name() {
        let mut ring = RingBuilder::new_in_memory().unwrap().build();

        ring.insert_user(&User::new("example", "example1@example.com").unwrap());
        ring.insert_user(&User::new("example", "example2@example.com").unwrap());
        ring.insert_user(&User::new("example", "example3@example.com").unwrap());

        let users = ring.get_user_where_name("example").unwrap();

        assert_eq!(users.len(), 3);

        for user in users {
            assert_eq!(user.name, "example");
        }
    }
}
