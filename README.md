mysql-repo
==========

A tiny library that exposes a `Repository` trait (and some helper traits) to structure some CRUD SQL boilerplate. 

Currently just for [rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple/).

Just implement some of the library's traits:

```rust
pub enum Users {}

impl Repository for Users {}

impl<'a> Insertable<Users> for &'a NewUser {
    fn sql() -> &'static str { "INSERT INTO users(email, data) VALUES (?, ?)" }
    fn to_params(&self) -> ::mysql::Params {
        (&self.email, &self.data).into()
    }
}

impl<'a> Updatable<Users> for &'a User {
    fn sql() -> &'static str { "UPDATE users SET email = ?, data = ? WHERE id = ?" }
    fn to_params(&self) -> ::mysql::Params {
        (&self.email, &self.data, &self.id).into()
    }
}

impl Findable<Users> for User {
    fn sql() -> &'static str { "SELECT id, email, data FROM users WHERE id = ? LIMIT 1" }
    fn from_row(row: ::mysql::Row) -> Result<Self, ::mysql::Error> {
        let (id, email, data) = ::mysql::from_row_opt(row)?;
        Ok(User { id, email, data, ..Default::default() })
    }
}

impl Whereable<Users> for User {
    fn wrapped_sql(sql: &str) -> String {
        ["SELECT id, email, data FROM users WHERE ", sql].concat()
    }
    fn from_row(row: ::mysql::Row) -> Result<Self, ::mysql::Error> {
        <Self as Findable<Users>>::from_row(row)
    }
}

impl Deletable<Users> for User {
    fn sql() -> &'static str { "DELETE FROM users WHERE id = ?" }
    fn to_params(&self) -> ::mysql::Params {
        (&self.id,).into()
    }
}
```

And then you can do:

```rust
let mut user: User = Repository::find(&mut conn, 1)?.ok_or("No users")?;    
user.email = "something@email.com";
Repository::update(&mut conn, &user)?;
let user: User = Repository::where_one("email = ?", ("something@email.com",))?.ok_or("No users")?;
```

etc.
