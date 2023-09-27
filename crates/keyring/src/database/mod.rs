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

    /// Select a user from the table users where the id matches. (exists)
    pub const SQL_SELECT_FROM_USERS_WHERE_EXISTS_ID: &str =
        "SELECT * FROM users WHERE EXISTS (SELECT * FROM users WHERE id = ?);";

    /// Select a user from the table users where the name matches. (exists)
    pub const SQL_SELECT_FROM_USERS_WHERE_EXISTS_NAME: &str =
        "SELECT * FROM users WHERE EXISTS (SELECT * FROM users WHERE name = ?);";

    /// Select a user from the table users where the email matches. (exists)
    pub const SQL_SELECT_FROM_USERS_WHERE_EXISTS_EMAIL: &str =
        "SELECT * FROM users WHERE EXISTS (SELECT * FROM users WHERE email = ?);";

    /// Select a user from the table users where the id and email matches. (exists)
    pub const SQL_SELECT_FROM_USERS_WHERE_EXISTS_ID_AND_EMAIL: &str =
        "SELECT * FROM users WHERE EXISTS (SELECT * FROM users WHERE id = ? AND email = ?);";

    /// Select all users from the table users
    pub const SQL_SELECT_ALL_FROM_USERS: &str = "SELECT * FROM users;";

    /// Select a user from the table users where the id matches
    pub const SQL_SELECT_FROM_USERS_WHERE_ID: &str = "SELECT * FROM users WHERE id = ?;";

    /// Select a user from the table users where the name matches
    pub const SQL_SELECT_FROM_USERS_WHERE_NAME: &str = "SELECT * FROM users WHERE name = ?;";

    /// Select a user from the table users where the email matches
    pub const SQL_SELECT_FROM_USERS_WHERE_EMAIL: &str = "SELECT * FROM users WHERE email = ?;";

    /// Delete a user from the table users where the id matches
    pub const SQL_DELETE_FROM_USERS_WHERE_ID: &str = "DELETE FROM users WHERE id = ?;";

    /// Delete a user from the table users where the name matches
    pub const SQL_DELETE_FROM_USERS_WHERE_NAME: &str = "DELETE FROM users WHERE name = ?;";

    /// Delete a user from the table users where the email matches
    pub const SQL_DELETE_FROM_USERS_WHERE_EMAIL: &str = "DELETE FROM users WHERE email = ?;";

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

    /// Select all private keys from the table private_keys
    pub const SQL_SELECT_ALL_FROM_PRIVATE_KEYS: &str = "SELECT * FROM private_keys;";

    /// Select a private key from the table private_keys where the user_id matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_USER_ID: &str =
        "SELECT * FROM private_keys WHERE user_id = ?;";

    /// Select a private key from the table private_keys where the primary_key matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_PRIMARY_KEY: &str =
        "SELECT * FROM private_keys WHERE primary_key = ?;";

    /// Select a private key from the table private_keys where the key_type matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_KEY_TYPE: &str =
        "SELECT * FROM private_keys WHERE key_type = ?;";

    /// Select a private key from the table private_keys where the expiry matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_EXPIRY: &str =
        "SELECT * FROM private_keys WHERE expiry = ?;";

    // pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_PRIVATE_KEY: &str = "SELECT * FROM private_keys WHERE private_key = ?;";

    /// Select a private key from the table private_keys where the fingerprint matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_FINGERPRINT: &str =
        "SELECT * FROM private_keys WHERE fingerprint = ?;";

    /// Select a private key from the table private_keys where the signature matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_WHERE_SIGNATURE: &str =
        "SELECT * FROM private_keys WHERE signature = ?;";

    /// Delete a private key from the table private_keys where the user_id matches
    pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_USER_ID: &str =
        "DELETE FROM private_keys WHERE user_id = ?;";

    /// Delete a private key from the table private_keys where the primary_key matches
    pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_PRIMARY_KEY: &str =
        "DELETE FROM private_keys WHERE primary_key = ?;";

    /// Delete a private key from the table private_keys where the key_type matches
    pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_KEY_TYPE: &str =
        "DELETE FROM private_keys WHERE key_type = ?;";

    /// Delete a private key from the table private_keys where the expiry matches
    pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_EXPIRY: &str =
        "DELETE FROM private_keys WHERE expiry = ?;";

    /// Delete a private key from the table private_keys where the fingerprint matches
    pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_FINGERPRINT: &str =
        "DELETE FROM private_keys WHERE fingerprint = ?;";

    // Delete a public key from the table private_keys where the signature matches
    // pub const SQL_DELETE_FROM_PRIVATE_KEYS_WHERE_SIGNATURE: &str = "DELETE FROM private_keys WHERE signature = ?;";

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

    /// Select all public keys from the table public_keys
    pub const SQL_SELECT_ALL_FROM_PUBLIC_KEYS: &str = "SELECT * FROM public_keys;";

    /// Select a public key from the table public_keys where the user_id matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_USER_ID: &str =
        "SELECT * FROM public_keys WHERE user_id = ?;";

    /// Select a public key from the table public_keys where the primary_key matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_PRIMARY_KEY: &str =
        "SELECT * FROM public_keys WHERE primary_key = ?;";

    /// Select a public key from the table public_keys where the key_type matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_KEY_TYPE: &str =
        "SELECT * FROM public_keys WHERE key_type = ?;";

    /// Select a public key from the table public_keys where the expiry matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_EXPIRY: &str =
        "SELECT * FROM public_keys WHERE expiry = ?;";

    /// Select a public key from the table public_keys where the public_key matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_PUBLIC_KEY: &str =
        "SELECT * FROM public_keys WHERE public_key = ?;";

    /// Select a public key from the table public_keys where the fingerprint matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_FINGERPRINT: &str =
        "SELECT * FROM public_keys WHERE fingerprint = ?;";

    /// Select a public key from the table public_keys where the signature matches
    pub const SQL_SELECT_FROM_PUBLIC_KEYS_WHERE_SIGNATURE: &str =
        "SELECT * FROM public_keys WHERE signature = ?;";

    /// Delete a public key from the table public_keys where the user_id matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_USER_ID: &str =
        "DELETE FROM public_keys WHERE user_id = ?;";

    /// Delete a public key from the table public_keys where the primary_key matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_PRIMARY_KEY: &str =
        "DELETE FROM public_keys WHERE primary_key = ?;";

    /// Delete a public key from the table public_keys where the key_type matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_KEY_TYPE: &str =
        "DELETE FROM public_keys WHERE key_type = ?;";

    /// Delete a public key from the table public_keys where the expiry matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_EXPIRY: &str =
        "DELETE FROM public_keys WHERE expiry = ?;";

    /// Delete a public key from the table public_keys where the public_key matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_PUBLIC_KEY: &str =
        "DELETE FROM public_keys WHERE public_key = ?;";

    /// Delete a public key from the table public_keys where the fingerprint matches
    pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_FINGERPRINT: &str =
        "DELETE FROM public_keys WHERE fingerprint = ?;";

    // Delete a public key from the table public_keys where the signature matches
    // pub const SQL_DELETE_FROM_PUBLIC_KEYS_WHERE_SIGNATURE: &str =
    //     "DELETE FROM public_keys WHERE signature = ?;";

    /// Select a private key from the table private_keys where the user_id and fingerprint matches
    pub const SQL_SELECT_FROM_PRIVATE_KEYS_AND_PUBLIC_KEYS_WHERE_USER_ID_AND_FINGERPRINT: &str =
"SELECT * FROM private_keys, public_keys WHERE private_keys.user_id = public_keys.user_id AND private_keys.fingerprint = public_keys.fingerprint AND private_keys.user_id = ? AND private_keys.fingerprint = ?;";
}
