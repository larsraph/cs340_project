use chrono::{NaiveDate, NaiveDateTime};
use leptos::prelude::*;
use mysql_async::prelude::*;
use mysql_async::Pool;
use serde::{Deserialize, Serialize};

async fn fetch_type<T>() -> Result<Vec<T>, ServerFnError>
where
    T: TableName + FromRow + Send + Sync + 'static,
{
    let mut conn = use_context::<Pool>().expect("global").get_conn().await?;

    let stmt = format!("SELECT * FROM {};", T::NAME);
    Ok(conn.exec(stmt, ()).await?)
}

#[server]
async fn fetch(ty: TableType) -> Result<Table, ServerFnError> {
    use TableType as TT;
    match ty {
        TT::Person => Ok(Table::Person(fetch_type::<Person>().await?)),
        TT::Clubs => Ok(Table::Clubs(fetch_type::<Clubs>().await?)),
        TT::Roles => Ok(Table::Roles(fetch_type::<Roles>().await?)),
        TT::Membership => Ok(Table::Membership(fetch_type::<Membership>().await?)),
        TT::Events => Ok(Table::Events(fetch_type::<Events>().await?)),
        TT::PhysicalEvents => Ok(Table::PhysicalEvents(fetch_type::<PhysicalEvents>().await?)),
        TT::VirtualEvents => Ok(Table::VirtualEvents(fetch_type::<VirtualEvents>().await?)),
        TT::Address => Ok(Table::Address(fetch_type::<Address>().await?)),
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TableType {
    Person,
    Clubs,
    Roles,
    Membership,
    Events,
    PhysicalEvents,
    VirtualEvents,
    Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Table {
    Person(Vec<Person>),
    Clubs(Vec<Clubs>),
    Roles(Vec<Roles>),
    Membership(Vec<Membership>),
    Events(Vec<Events>),
    PhysicalEvents(Vec<PhysicalEvents>),
    VirtualEvents(Vec<VirtualEvents>),
    Address(Vec<Address>),
}

pub trait TableName {
    const NAME: &str;
}

macro_rules! table_name {
    ($ty:ty, $name:literal) => {
        impl TableName for $ty {
            const NAME: &str = $name;
        }
    };
}

table_name!(Person, "People");
table_name!(Clubs, "Clubs");
table_name!(Roles, "Roles");
table_name!(Membership, "Membership");
table_name!(Events, "Events");
table_name!(PhysicalEvents, "PhysicalEvents");
table_name!(VirtualEvents, "VirtualEvents");
table_name!(Address, "Addresses");

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    person_id: i32,
    email: String,
    onid: String,
    phone_number: String,
    date_of_birth: NaiveDate,
    address_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Clubs {
    club_id: i32,
    name: String,
    date_created: NaiveDate,
    is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Roles {
    role_id: i32,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Membership {
    person_id: i32,
    role_id: i32,
    club_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Events {
    event_id: i32,
    name: String,
    description: String,
    time_start: NaiveDateTime,
    time_end: NaiveDateTime,
    club_id: i32,
    organizer_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct PhysicalEvents {
    event_id: i32,
    address_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct VirtualEvents {
    event_id: i32,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Address {
    address_id: i32,
    country_code: String,
    zip_code: String,
    address_ln1: String,
    address_ln2: Option<String>,
    city: Option<String>,
    state: Option<String>,
}
