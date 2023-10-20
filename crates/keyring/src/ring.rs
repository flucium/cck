use std::path::Path;

use rusqlite as sqlite;

use crate::{
    sql::{self, SQL_INSERT_INTO_PRIVATE_KEYS, SQL_INSERT_INTO_PUBLIC_KEYS},
    Expiry, Key, KeyType, PrivateKey, PublicKey, User,
};

/// RingBuilder
pub struct RingBuilder(Ring);

impl RingBuilder {
    /// RingBuilder
    ///
    /// # Example
    /// ```
    /// let ring_builder = RingBuilder::new("./database");
    /// ```
    pub fn new(path: &Path) -> cck_common::Result<Self> {
        let mut ring = Ring::open(path)?;

        init_tables(&mut ring.0)?;

        Ok(Self(ring))
    }

    /// RingBuilder
    ///
    /// # Example
    /// ```
    /// let mut ring_builder = RingBuilder::new_in_memory();
    /// ```
    pub fn new_in_memory() -> cck_common::Result<Self> {
        let mut ring = Ring::open_in_memory()?;

        init_tables(&mut ring.0)?;

        Ok(Self(ring))
    }

    // Todo!
    // pub fn password(&mut self, password: impl AsRef<[u8]>) -> cck_common::Result<&mut Self> {

    //     Ok(self)
    // }

    /// Ring build
    ///
    /// # Example
    /// ```
    /// let ring = RingBuilder::new("./database").build();
    /// ```
    pub fn build(self) -> Ring {
        self.0
    }
}

/// Ring
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

    /*
        Insert
    */

    pub fn insert_user(&mut self, user: impl Into<User>) -> cck_common::Result<()> {
        let user = user.into();
        self.0
            .execute(
                sql::SQL_INSERT_INTO_USERS,
                sqlite::params![user.id(), user.name(), user.email()],
            )
            .map_err(|_| cck_common::Error)?;

        Ok(())
    }

    pub fn insert_private_key(
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

    pub fn insert_public_key(
        &mut self,
        user: impl Into<User>,
        public_key: impl Into<PublicKey>,
    ) -> cck_common::Result<()> {
        let user = user.into();
        let public_key = public_key.into();
        self.0
            .execute(
                SQL_INSERT_INTO_PUBLIC_KEYS,
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

    /*
        Get
    */

    pub fn get_user_from_id(&self, id: impl Into<String>) -> cck_common::Result<User> {
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

    pub fn get_user_from_email(&self, email: impl Into<String>) -> cck_common::Result<User> {
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

    pub fn get_users_from_name(&self, name: impl Into<String>) -> cck_common::Result<Vec<User>> {
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

    pub fn get_private_key_from_user_and_fingerprint(
        &self,
        user: impl Into<User>,
        fingerpring: impl Into<String>,
    ) -> cck_common::Result<PrivateKey> {
        let user = user.into();
        let fingerpring = fingerpring.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_FINGERPRINT)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![user.id(), fingerpring], |row| {
                Ok(PrivateKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    private_key: row.get(3)?,
                    public_key: Vec::default(),
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        let private_key = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(private_key)
    }

    pub fn get_private_keys_from_user(
        &self,
        user: impl Into<User>,
    ) -> cck_common::Result<Vec<PrivateKey>> {
        let user = user.into();

        let mut private_keys = Vec::new();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_USER_ID)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![user.id()], |row| {
                Ok(PrivateKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    private_key: row.get(3)?,
                    public_key: Vec::default(),
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        while let Some(private_key) = rows.next() {
            private_keys.push(private_key.map_err(|_| cck_common::Error)?);
        }

        Ok(private_keys)
    }

    pub fn get_priavte_key_from_fingerprint(
        &self,
        fingerpring: impl Into<String>,
    ) -> cck_common::Result<PrivateKey> {
        let fingerpring = fingerpring.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_FINGERPRINT)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![fingerpring], |row| {
                Ok(PrivateKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    private_key: row.get(3)?,
                    public_key: Vec::default(),
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        let private_key = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(private_key)
    }

    pub fn get_public_key_from_user_and_fingerprint(
        &self,
        user: impl Into<User>,
        fingerpring: impl Into<String>,
    ) -> cck_common::Result<PublicKey> {
        let user = user.into();
        let fingerpring = fingerpring.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_FINGERPRINT)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![user.id(), fingerpring], |row| {
                Ok(PublicKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    public_key: row.get(3)?,
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        let public_key = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(public_key)
    }

    pub fn get_public_keys_from_user(
        &self,
        user: impl Into<User>,
    ) -> cck_common::Result<Vec<PublicKey>> {
        let user = user.into();

        let mut public_keys = Vec::new();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_USER_ID)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![user.id()], |row| {
                Ok(PublicKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    public_key: row.get(3)?,
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        while let Some(public_key) = rows.next() {
            public_keys.push(public_key.map_err(|_| cck_common::Error)?);
        }

        Ok(public_keys)
    }

    pub fn get_public_key_from_fingerprint(
        &self,
        fingerpring: impl Into<String>,
    ) -> cck_common::Result<PublicKey> {
        let fingerpring = fingerpring.into();

        let mut stmt = self
            .0
            .prepare(sql::SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_FINGERPRINT)
            .map_err(|_| cck_common::Error)?;

        let mut rows = stmt
            .query_map(sqlite::params![fingerpring], |row| {
                Ok(PublicKey {
                    primary: row.get(0)?,
                    key_type: KeyType::from_string(row.get(1)?).unwrap(),
                    expiry: Expiry::from_string(row.get(2)?).unwrap(),
                    public_key: row.get(3)?,
                    fingerprint: row.get(4)?,
                    signature: match row.get(5) {
                        Err(_) => None,
                        Ok(signature) => Some(signature),
                    },
                })
            })
            .map_err(|_| cck_common::Error)?;

        let public_key = rows
            .next()
            .ok_or(cck_common::Error)?
            .map_err(|_| cck_common::Error)?;

        Ok(public_key)
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

// fn drop_tables(conn: &sqlite::Connection) -> cck_common::Result<()> {
//     conn.execute(sql::SQL_DROP_TABLE_USERS, [])
//         .map_err(|_| cck_common::Error)?;

//     conn.execute(sql::SQL_DROP_TABLE_PRIVATE_KEYS, [])
//         .map_err(|_| cck_common::Error)?;

//     conn.execute(sql::SQL_DROP_TABLE_PUBLIC_KEYS, [])
//         .map_err(|_| cck_common::Error)?;

//     Ok(())
// }

// fn remove_database_file(path: &Path) -> cck_common::Result<()> {
//     if path.exists() {
//         std::fs::remove_file(path).map_err(|_| cck_common::Error)
//     } else if path.is_dir() {
//         Err(cck_common::Error)
//     } else {
//         Err(cck_common::Error)
//     }
// }
