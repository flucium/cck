use std::path::Path;

use rusqlite as sqlite;

use crate::{
    sql::{self, SQL_INSERT_INTO_PRIVATE_KEYS},
    Key, PrivateKey, PublicKey, User, key_type,
};

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

    fn insert_private_key(
        &mut self,
        user: impl Into<User>,
        private_key: impl Into<PrivateKey>,
    ) -> cck_common::Result<()> {
        let user = user.into();
        let private_key = private_key.into();
        self.0
            .execute(
                SQL_INSERT_INTO_PRIVATE_KEYS,
                sqlite::params![
                    user.id(),
                    private_key.key_type().to_string(),
                    private_key.expiry().to_string(),
                    private_key.as_bytes(),
                    private_key.fingerprint(),
                    private_key.signature()
                ],
            )
            .map_err(|_| cck_common::Error)?;

        Ok(())
    }

    fn insert_public_key(
        &mut self,
        user: impl Into<User>,
        public_key: impl Into<PublicKey>,
    ) -> cck_common::Result<()> {
        let user = user.into();
        let public_key = public_key.into();
        self.0
            .execute(
                SQL_INSERT_INTO_PRIVATE_KEYS,
                sqlite::params![
                    user.id(),
                    public_key.key_type().to_string(),
                    public_key.expiry().to_string(),
                    public_key.as_bytes(),
                    public_key.fingerprint(),
                    public_key.signature()
                ],
            )
            .map_err(|_| cck_common::Error)?;

        Ok(())
    }

    fn get_user_from_id(&self, id: impl Into<String>) -> cck_common::Result<User> {
        let id = id.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_USERS_WHERE_ID)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![id], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                })
            })
            .map_err(|_| cck_common::Error)?;

        let user = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(user)
    }

    fn get_user_from_email(&self, email: impl Into<String>) -> cck_common::Result<User> {
        let email = email.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_USERS_WHERE_EMAIL)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![email], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                })
            })
            .map_err(|_| cck_common::Error)?;

        let user = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(user)
    }

    fn get_users_from_name(&self, name: impl Into<String>) -> cck_common::Result<Vec<User>> {
        let name = name.into();

        let mut users = Vec::new();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_USERS_WHERE_NAME)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![name], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                })
            })
            .map_err(|_| cck_common::Error)?;

        while let Some(user) = rows.next() {
            users.push(user.map_err(|_| cck_common::Error)?);
        }

        Ok(users)
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
