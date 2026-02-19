use std::hash::Hash;

use leptos::html::{HtmlElement, Input};
use leptos::prelude::*;
use serde::de::value;

use crate::tables::TableType;

/// Heres the pipeline for rendering an editable sheet:
///
/// 1. Req -> Vec<T> where each field of T is a column in the table.
/// 2. Turn T into a structure of T where each element is a signal.
/// 3. Render the table using the getters + a serializer for each field. In other words we need a way to take a T.elem(n) and serialize it.
///
/// Selection
/// 1. (for now) we store a single Option<(T::Key, usize)> signal to represent the selected cell.
/// 2. On selection we need to highlight or embolden a cell.
/// 3. We also can enter edit mode.
/// 4. If a cell receives an edit-mode signal AND a selection signal then it should render an input box with the current value of the cell.
/// 5. On submission of the input box we must use the (_, usize) to identify the deserializer for the field and attempt to deserialize.
///     ERRORS: See 6.errors and just do the same thing.
/// 6. If it properly deserialized then we send a server request to write the change to the database.
///     IMPORTANT: the DB might contain extra constaints. We must display any DB errors to the user.
///     If such a error occurs then we will need to do one of the following:
///         a. Revert to the previous state (just don't save it)
///         b. Don't exit edit mode and force a valid value (with a keycombo such as exit-exit forcing revert)
///         c. allow invalid values but highlight them in red (extra metadata per cell).
/// 7. If it properly deserialized and there was no DB error then we use the (T::Key, usize) to find the proper write signal to update the cell.
///
/// NOTE: The cells must be reactive BUT they might not need to be reactive to a signal.
/// If we make a cell reactive to selection + edits then I think we can skip the step of making T into a structure of signals. Instead we would load each elem of T once into a derived signal that uses logic based on the select and edit signal to update the variable.
/// I think this might be possible... BUT I don't know exactly how errors will work with this.
///
/// Navigation
/// 1. Rows are keyed by T::Key because thats logical BUT if we've selected row R and we want to find the next row down then I have no clue how we do that.
/// 2. One option is to not key the rows.
/// 3. The other option is to maintain a separate BTreeSet<T::Key> that we use for navigation. This is probably the best.
///
/// Speaking of sorting it's going to be exclusivly sorted asc by T::Key for now. Also T::Key should probably be immutable
///     BUT with my validation protocol I think it would be easier to let the DB handle any uniqueness errors.
///
/// Creation
/// Floating cell at the bottom of the table that you treat like all other cells BUT when you submit it creates a new row.
/// 1. The main issue I see is that this is only compatible with the "bad cells are highlighted red" error protocol because all the cells will be initially invalid.
/// 2. Another issues is that we need to batch all edits of a new row until 1. All cells are valid.
/// 3. Also how would auto generated values like most PKs work? This is incompatible with my current model of Client handling loose types and Server handling strong types (and constraints).
///     For this to work properly we would need to serailize a loose schema and give it to the client.
/// 3.1. Actually this could work: I just need to be able to A. Mark rows as defaultable (ex NULL) B. mark rows as AUTO_INCREMENT and handle that in the client. This is still pretty annoying though.
///
/// Current solution:
/// 1. For now I just will NOT support AUTO_INCREMENT or DEFAULT values.
/// To finalize the vision: There will always be a floating "new row" at the bottom of the table. What will it's KEY be? I think I'll encode Key as an enum with one of the states being NEW and including it in my BTreeMap.
///     On submission of an edit IF we're in a Key::New then we must create a new row in the sheet, FUCK what will it's key by if it's not initalized?
///     Ok maybe we do include an ability to auto_increment keys? But what about keys that aren't AU?
///     Ok ok ok. We are NOT going to create a new row UNTIL we submit the row to the DB and it gets accepted.
///     Instead we must mark all other fields as invalid (every submission to a NEW row much check if all are valid and submit if so).
///     On submission with a valid PK we THEN create a new row. We either A. Have mutable keys and update the old key or B. Copy to to a new row and ZERO.
/// First edit -> Mark all fields as invalid
/// Following edits -> Check if all fields are still invalid
/// Valid submission -> Create row and make new NEW row.
///
/// Deletion
/// Option 1: Have any ROW with no elements be automatically deleted (except the new row)
/// Options 2: Have a delete button next to all rows which upon being clicked submits a server fn call using it's preprovided Key.
/// IMPORTANT: We must handle any DB errors.
///
/// Option 2 is currently the go to.
///
/// Joins
/// So a joined table is in my head represented as a table where one of the rows contains another table (which may have only one ROW or many).
/// But this is incredibly complex because MySql flattens joins (pretty weird IMO).
///
/// In a flat table many different fields point to the same value. Our current UI doesn't support this. BUT it *could*.
/// The nature of a join is that there would be multiple keys associated with each ROW but each value would only be associated with one key.
/// So basically what I'm saying is it's not in scope but to keep it in mind for the future.
///
/// validation... So If i am validating do I really need to deserialize? As long as the server serialization and client serialization are the same I can
struct Docs;

trait Row {
    #[cfg(feature = "ssr")]
    const TABLE_NAME: &'static str;

    const FIELDS: &'static [&'static str];

    const KEY_INDEX: usize;
    const KEY_FIELD: &'static str = Self::FIELDS[Self::KEY_INDEX];

    type Key: Copy + PartialEq + Into<mysql_async::Value> + ToString + Send + 'static;

    fn key(&self) -> Self::Key;

    fn ser_field(&self, field: usize) -> String;

    fn validate(field: usize, value: &str) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
struct IdMaybeNew<K> {
    key: Option<K>,
    field: usize,
}

struct Id<K> {
    key: K,
    field: usize,
}

#[component]
fn Cell<'r, R: Row>(
    row: Option<&'r R>,
    field: usize,
    selected: impl Fn() -> Option<Id<R::Key>> + Copy + Send + 'static,
    editing: impl Fn() -> bool + Send + 'static,
) -> impl IntoView {
    let id = Some(Id {
        key: row.map(|r| r.key()),
        field,
    });
    let selected = move || selected() == id;

    let (value, set_value) = signal(row.map(|r| r.ser_field(field)).unwrap_or_default());
    let (valid, set_valid) = signal(true);

    let node_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = node_ref
            .get()
            .expect("must be mounted to fire event")
            .value();

        let cl_valid = R::validate(field, &value);

        set_value(value);

        // BUT WAIT THATS ASYNC!
        let db_valid = if cl_valid { todo!("server req") } else { false };

        set_valid(cl_valid && db_valid);
    };

    view! {
        <td class:selected=selected class:valid=valid>
            {move || {
                if editing() && selected() {
                    view! { <input type="text" value=value node_ref=node_ref on:submit=submit /> }.into_any()
                } else {
                    value.into_any()
                }
            }}
        </td>
    }
}

#[cfg(feature = "ssr")]
async fn _update_cell<T: Row>(id: Id<T::Key>, value: String) -> Result<(), ServerFnError> {
    use mysql_async::prelude::*;
    use mysql_async::Pool;

    let mut conn = use_context::<Pool>().expect("global").get_conn().await?;
    let stmt = format!(
        "UPDATE {} SET {} = ? WHERE {} = ?;",
        T::TABLE_NAME,
        T::FIELDS[id.field],
        T::KEY_FIELD
    );

    conn.exec_drop(stmt, (value, id.key)).await?;

    Ok(())
}

// #[server]
// async fn update_cell(tt: TableType, key: T::Key, field: usize, value: String) -> Result<(), ServerFnError> {}

trait Sheetable {
    type Key: Clone + Copy + Eq + Hash + Send + Sync + 'static;

    fn key(&self) -> Self::Key;

    const FIELDS: &'static [&'static str];

    fn serialize_fields(&self) -> impl Iterator<Item = Box<dyn Trait + '_>> + '_;

    fn deserialize_field(&mut self, index: usize, value: &str);
}

// what is a sheet, it is a table of data where we have any number of selections and we can go into edit mode to write directly into the selections. Then upon exiting edit mode we validate data type. There are also controls for all of these operations.

// MVP: table of data, single selection @ all times, edit mode.

struct ExSheetable {
    id: i32,
    name: String,
}

trait Trait: Fn() -> String + Send {}

impl<T: Fn() -> String + Send> Trait for T {}

impl Sheetable for ExSheetable {
    type Key = i32;

    fn key(&self) -> Self::Key {
        self.id
    }

    const FIELDS: &'static [&'static str] = &["id", "name"];

    fn serialize_fields(&self) -> impl Iterator<Item = Box<dyn Trait + '_>> + '_ {
        let id: Box<dyn Trait> = Box::new(|| self.id.to_string());
        let name: Box<dyn Trait> = Box::new(|| self.name.clone());
        [id, name].into_iter()
    }

    fn deserialize_field(&mut self, index: usize, value: &str) {
        match index {
            0 => self.id = value.parse().unwrap_or(self.id),
            1 => self.name = value.to_string(),
            _ => (),
        }
    }
}

#[component]
pub fn Sheet<T, F>(data: impl Fn() -> Vec<F> + Send + 'static) -> impl IntoView
where
    T: Sheetable + Eq + Send + 'static,
    F: Fn() -> T + Send + 'static + Copy,
{
    let (selection, set_selection) = signal(None);
    let (editing, set_editing) = signal(false);

    view! {
        <table class="sheet">
            <thead>
                <tr>{T::FIELDS.iter().map(|field| view! { <th>{*field}</th> }).collect_view()}</tr>
            </thead>
            <tbody>
                <For
                    each=data
                    key=|f| f().key()
                    children=move |v| {
                        let fields = v()
                            .serialize_fields()
                            .enumerate()
                            .map(|(col, ser)| {
                                view! {
                                    <SheetCell id=move || (v().key(), col) selection=selection>
                                        {ser}
                                    </SheetCell>
                                }
                            })
                            .collect_view();
                        view! { <tr>{fields}</tr> }
                    }
                />
            </tbody>
        </table>
    }
}

#[component]
fn SheetCell<K>(
    id: impl Fn() -> (K, usize) + Send + 'static,
    selection: impl Fn() -> Option<(K, usize)> + Send + 'static,
    children: Children,
) -> impl IntoView
where
    K: PartialEq + Clone + Copy + Send + 'static,
{
    view! { <td class:selected=move || { selection() == Some(id()) }>{children()}</td> }
}
