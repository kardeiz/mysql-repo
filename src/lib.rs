extern crate mysql;

use mysql::{Params, Row, Error};
use mysql::conn::GenericConnection;

pub trait Repository: Sized {

    fn where_one<T, P, Where>(conn: &mut T, sql: &str, params: P) -> Result<Option<Where>, Error>
        where T: GenericConnection, Where: Whereable<Self>, P: Into<Params>, Self: Sized {
        let out = conn.prep_exec(Where::wrapped_sql(sql), params.into())?
            .flat_map(|x| x)
            .flat_map(|x| Where::from_row(x) )
            .next();
        Ok(out)
    }

    fn where_all<T, P, Where>(conn: &mut T, sql: &str, params: P) -> Result<Vec<Where>, Error>
        where T: GenericConnection, Where: Whereable<Self>, P: Into<::mysql::Params>, Self: Sized {
        let out = conn.prep_exec(Where::wrapped_sql(sql), params.into())?
            .flat_map(|x| x)
            .flat_map(|x| Where::from_row(x) )
            .collect();
        Ok(out)
    }

    fn find<T, Find>(conn: &mut T, id: u64) 
        -> Result<Option<Find>, Error> where T: GenericConnection, Find: Findable<Self>, Self: Sized {
        let out = conn.prep_exec(Find::sql(), (id,))?
            .flat_map(|x| x)
            .flat_map(|x| Find::from_row(x) )
            .next();
        Ok(out)
    }

    fn insert<T, Ins>(conn: &mut T, obj: Ins) 
        -> Result<u64, Error> where T: GenericConnection, Ins: Insertable<Self>, Self: Sized {
        let res = conn.prep_exec(Ins::sql(), obj.to_params())?;
        let id = res.last_insert_id();
        Ok(id)
    }

    fn update<T, Up>(conn: &mut T, obj: Up) 
        -> Result<(), Error> where T: GenericConnection, Up: Updatable<Self>, Self: Sized {
        conn.prep_exec(Up::sql(), obj.to_params())?;
        Ok(())
    }

    fn delete<T, Del>(conn: &mut T, obj: Del) 
        -> Result<(), Error> where T: GenericConnection, Del: Deletable<Self>, Self: Sized {
        conn.prep_exec(Del::sql(), obj.to_params())?;
        Ok(())
    }
}

pub trait Whereable<R: Repository>: Sized {
    fn wrapped_sql(sql: &str) -> String;
    fn from_row(row: Row) -> Result<Self, Error>;
}

pub trait Insertable<R: Repository> {
    fn sql() -> &'static str;
    fn to_params(&self) -> Params;
}

pub trait Updatable<R: Repository>: Sized {
    fn sql() -> &'static str;
    fn to_params(&self) -> Params;
}

pub trait Findable<R: Repository>: Sized {
    fn sql() -> &'static str;
    fn from_row(row: Row) -> Result<Self, Error>;
}

pub trait Deletable<R: Repository> {
    fn sql() -> &'static str;
    fn to_params(&self) -> Params;
}

