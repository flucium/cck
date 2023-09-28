mod sqlite;

mod sql {
    /*
        SQL statements for the database
        Table: users
    */
    /// Create the table users if it does not exist
    pub const SQL_CREATE_TABLE_USERS: &str =
        "CREATE TABLE IF NOT EXISTS users (id TEXT, name TEXT, email TEXT);";

    /// Drop the table users
    pub const SQL_DROP_TABLE_USERS: &str = "DROP TABLE users;";

    /// Insert a new user into the table users
    pub const SQL_INSERT_INTO_USERS: &str = "INSERT INTO users (id, name, email) VALUES(?, ?, ?);";

    /// Select a user from the table users where the id matches
    ///
    /// `SQL` - SELECT * FROM users WHERE id = ?;
    pub const SQL_SELECT_FROM_USERS_WHERE_ID: &str = "SELECT * FROM users WHERE id = ?;";

    /// Select a user from the table users where the name matches
    ///
    /// `SQL` - SELECT * FROM users WHERE name = ?;
    pub const SQL_SELECT_FROM_USERS_WHERE_NAME: &str = "SELECT * FROM users WHERE name = ?;";

    /// Select a user from the table users where the email matches
    ///
    /// `SQL` - SELECT * FROM users WHERE email = ?;
    pub const SQL_SELECT_FROM_USERS_WHERE_EMAIL: &str = "SELECT * FROM users WHERE email = ?;";

    /*
    SQL statements for the database
    Table: private_keys
    */
    /// Create the table private_keys if it does not exist
    pub const SQL_CREATE_TABLE_PRIVATE_KEYS: &str = "CREATE TABLE IF NOT EXISTS private_keys (user_id TEXT, primary_key boolean, key_type TEXT, expiry TEXT, private_key TEXT, fingerprint TEXT, signature TEXT);";

    /// Drop the table private_keys
    pub const SQL_DROP_TABLE_PRIVATE_KEYS: &str = "DROP TABLE private_keys;";

    /// Insert a new private key into the table private_keys
    pub const SQL_INSERT_INTO_PRIVATE_KEYS: &str = "INSERT INTO private_keys (user_id, primary_key, key_type, expiry, private_key, fingerprint, signature) VALUES(?, ?, ?, ?, ?, ?, ?);";


    /// Select a private key from the table private_keys where the fingerprint matches
    ///
    /// `SQL` - SELECT * FROM private_keys WHERE fingerprint = ?;
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_FINGERPRINT: &str =
        "SELECT * FROM private_keys WHERE fingerprint = ?;";

    /// Select a private key from the table private_keys where the user_id and fingerprint matches
    ///
    /// `SQL` - SELECT * FROM private_keys WHERE user_id = ? AND fingerprint = ?;
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_USER_ID_AND_FINGERPRINT: &str =
        "SELECT * FROM private_keys WHERE user_id = ? AND fingerprint = ?;";

    /*
    SQL statements for the database
    Table: public_keys
    */
    /// Create the table public_keys if it does not exist
    pub const SQL_CREATE_TABLE_PUBLIC_KEYS: &str = "CREATE TABLE IF NOT EXISTS public_keys (user_id TEXT, primary_key boolean, key_type TEXT, expiry TEXT, public_key TEXT, fingerprint TEXT, signature TEXT);";

    /// Drop the table public_keys
    pub const SQL_DROP_TABLE_PUBLIC_KEYS: &str = "DROP TABLE public_keys;";

    /// Insert a new public key into the table public_keys
    pub const SQL_INSERT_INTO_PUBLIC_KEYS: &str = "INSERT INTO public_keys (user_id, primary_key, key_type, expiry, public_key, fingerprint, signature) VALUES(?, ?, ?, ?, ?, ?, ?);";

    /// Select a public key from the table public_keys where the fingerprint matches
    /// 
    /// `SQL` - SELECT * FROM public_keys WHERE fingerprint = ?;
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_FINGERPRINT: &str =
        "SELECT * FROM public_keys WHERE fingerprint = ?;";

    /// Select a public key from the table public_keys where the user_id and fingerprint matches
    ///
    /// `SQL` - SELECT * FROM public_keys WHERE user_id = ? AND fingerprint = ?;
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_USER_ID_AND_FINGERPRINT: &str =
        "SELECT * FROM public_keys WHERE user_id = ? AND fingerprint = ?;";
    
}
