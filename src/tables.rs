use chrono::{NaiveDate, NaiveDateTime};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

#[server]
pub async fn fetch(ty: TableType) -> Result<Table, ServerFnError> {
    use mysql_async::prelude::*;
    use mysql_async::Pool;
    use {Table as TB, TableType as TT};

    async fn _fetch<T>() -> Result<Vec<T>, ServerFnError>
    where
        T: TableTrait + FromRow + Send + Sync + 'static,
    {
        let mut conn = use_context::<Pool>().expect("global").get_conn().await?;

        let stmt = format!("SELECT * FROM {};", T::NAME);
        Ok(conn.exec(stmt, ()).await?)
    }

    match ty {
        TT::Person => Ok(TB::Person(_fetch::<Person>().await?)),
        TT::Clubs => Ok(TB::Clubs(_fetch::<Clubs>().await?)),
        TT::Roles => Ok(TB::Roles(_fetch::<Roles>().await?)),
        TT::Membership => Ok(TB::Membership(_fetch::<Membership>().await?)),
        TT::Events => Ok(TB::Events(_fetch::<Events>().await?)),
        TT::PhysicalEvents => Ok(TB::PhysicalEvents(_fetch::<PhysicalEvents>().await?)),
        TT::VirtualEvents => Ok(TB::VirtualEvents(_fetch::<VirtualEvents>().await?)),
        TT::Address => Ok(TB::Address(_fetch::<Address>().await?)),
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

impl Table {
    pub fn view(self) -> impl IntoView {
        use Table as TB;
        match self {
            TB::Person(mut data) => view! { <Table head=Person::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::Clubs(mut data) => view! { <Table head=Clubs::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::Roles(mut data) => view! { <Table head=Roles::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::Membership(mut data) => view! { <Table head=Membership::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::Events(mut data) => view! { <Table head=Events::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::PhysicalEvents(mut data) => view! { <Table head=PhysicalEvents::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::VirtualEvents(mut data) => view! { <Table head=VirtualEvents::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
            TB::Address(mut data) => view! { <Table head=Address::head() body=data.drain(..).map(TableTrait::data) /> }
            .into_any(),
        }
    }
}

#[component]
pub fn Table(head: impl IntoView, body: impl IntoIterator<Item: IntoView>) -> impl IntoView {
    view! {
        <table>
            <thead>{head}</thead>
            <tbody>{body.into_iter().collect_view()}</tbody>
        </table>
    }
}

trait TableTrait {
    #[cfg(feature = "ssr")]
    const NAME: &str;

    fn head() -> impl IntoView;

    fn data(self) -> impl IntoView;
}

macro_rules! table_trait {
    ($ty:ty, $name:literal, [$( $field:ident ),*]) => {
        impl TableTrait for $ty {
            #[cfg(feature = "ssr")]
            const NAME: &str = $name;

            fn head() -> impl IntoView {
                view! {
                    <tr>
                        $(
                            <th>{stringify!($field)}</th>
                        )*
                    </tr>
                }
            }

            fn data(self) -> impl IntoView {
                view! {
                    <tr>
                        $(
                            <td>{move || format!("{:?}", self.$field)}</td>
                        )*
                    </tr>
                }
            }
        }
    };
}

table_trait!(
    Person,
    "People",
    [
        person_id,
        email,
        onid,
        phone_number,
        date_of_birth,
        address_id
    ]
);
table_trait!(Clubs, "Clubs", [club_id, name, date_created, is_active]);
table_trait!(Roles, "Roles", [role_id, name]);
table_trait!(Membership, "Membership", [person_id, role_id, club_id]);
table_trait!(
    Events,
    "Events",
    [
        event_id,
        name,
        description,
        time_start,
        time_end,
        club_id,
        organizer_id
    ]
);
table_trait!(PhysicalEvents, "PhysicalEvents", [event_id, address_id]);
table_trait!(VirtualEvents, "VirtualEvents", [event_id, url]);
table_trait!(
    Address,
    "Addresses",
    [
        address_id,
        country_code,
        zip_code,
        address_ln1,
        address_ln2,
        city,
        state
    ]
);

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Person {
    person_id: i32,
    email: String,
    onid: String,
    phone_number: String,
    date_of_birth: NaiveDate,
    address_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Clubs {
    club_id: i32,
    name: String,
    date_created: NaiveDate,
    is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Roles {
    role_id: i32,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Membership {
    person_id: i32,
    role_id: i32,
    club_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Events {
    event_id: i32,
    name: String,
    description: String,
    time_start: NaiveDateTime,
    time_end: NaiveDateTime,
    club_id: i32,
    organizer_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct PhysicalEvents {
    event_id: i32,
    address_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct VirtualEvents {
    event_id: i32,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsSlice)]
#[cfg_attr(feature = "ssr", derive(mysql_async::prelude::FromRow))]
pub struct Address {
    address_id: i32,
    country_code: String,
    zip_code: String,
    address_ln1: String,
    address_ln2: Option<String>,
    city: Option<String>,
    state: Option<String>,
}
