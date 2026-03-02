// mod types {
//     use std::mem::MaybeUninit;

//     use bitvec::prelude::*;
//     use chrono::{NaiveDate, NaiveDateTime};
//     use serde::{Deserialize, Serialize};
//     use {Type as T, Value as V, ValueColumnInner as C};

//     // should expand to become
//     // #[derive(Clone, Debug, Serialize, Deserialize)]
//     // pub enum Value {
//     //     U8(u8),
//     //     I32(i32),
//     //     String(String),
//     //     Date(NaiveDate),
//     //     DateTime(NaiveDateTime),
//     // }

//     // typed! {
//     //     #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
//     //     pub struct Type();
//     // }
//     // // should expand to become
//     // #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
//     // pub enum Type {
//     //     U8,
//     //     I32,
//     //     Date,
//     //     DateTime,
//     //     String,
//     // }

//     // typed! {
//     //     #[derive(Debug)]
//     //     pub struct ValueVec(Vec<$ty>);
//     // }
//     // // should expand to become
//     // #[derive(Debug)]
//     // pub enum ValueVec {
//     //     U8(Vec<u8>),
//     //     I32(Vec<i32>),
//     //     String(Vec<String>),
//     //     Date(Vec<NaiveDate>),
//     //     DateTime(Vec<NaiveDateTime>),
//     // }

//     #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
//     pub enum Type {
//         U8,
//         I32,
//         Date,
//         DateTime,
//         String,
//     }

//     #[derive(Clone, Debug, Serialize, Deserialize)]
//     pub enum Value {
//         U8(u8),
//         I32(i32),
//         String(String),
//         Date(NaiveDate),
//         DateTime(NaiveDateTime),
//     }

//     pub enum ValueRef<'a> {
//         U8(&'a u8),
//         I32(&'a i32),
//         String(&'a str),
//         Date(&'a NaiveDate),
//         DateTime(&'a NaiveDateTime),
//     }

//     #[derive(Debug)]
//     pub enum ValueColumnInner {
//         U8(Vec<MaybeUninit<u8>>),
//         I32(Vec<MaybeUninit<i32>>),
//         String(Vec<MaybeUninit<String>>),
//         Date(Vec<MaybeUninit<NaiveDate>>),
//         DateTime(Vec<MaybeUninit<NaiveDateTime>>),
//     }

//     impl ValueColumnInner {
//         pub fn push_uninit(&mut self) {
//             match self {
//                 C::U8(col) => col.push(MaybeUninit::uninit()),
//                 C::I32(col) => col.push(MaybeUninit::uninit()),
//                 C::String(col) => col.push(MaybeUninit::uninit()),
//                 C::Date(col) => col.push(MaybeUninit::uninit()),
//                 C::DateTime(col) => col.push(MaybeUninit::uninit()),
//             }
//         }

//         pub fn len(&self) -> usize {
//             match self {
//                 C::U8(col) => col.len(),
//                 C::I32(col) => col.len(),
//                 C::String(col) => col.len(),
//                 C::Date(col) => col.len(),
//                 C::DateTime(col) => col.len(),
//             }
//         }
//     }

//     #[derive(Debug)]
//     pub struct ValueColumn {
//         pub inner: ValueColumnInner,
//         pub nulls: Option<BitVec>,
//     }

//     impl ValueColumn {
//         pub fn push(&mut self, value: Option<Value>) {
//             fn _push<T>(col: &mut Vec<MaybeUninit<T>>, value: T, nulls: &mut Option<BitVec>) {
//                 col.push(MaybeUninit::new(value));
//                 if let Some(nulls) = nulls {
//                     nulls.push(false);
//                 }
//             }

//             match (value, &mut self.inner, &mut self.nulls) {
//                 (Some(V::U8(v)), C::U8(c), n) => _push(c, v, n),
//                 (Some(V::I32(v)), C::I32(c), n) => _push(c, v, n),
//                 (Some(V::String(v)), C::String(c), n) => _push(c, v, n),
//                 (Some(V::Date(v)), C::Date(c), n) => _push(c, v, n),
//                 (Some(V::DateTime(v)), C::DateTime(c), n) => _push(c, v, n),
//                 (None, c, Some(n)) => {
//                     c.push_uninit();
//                     n.push(true);
//                 }
//                 _ => panic!("value type does not match column type"),
//             }
//         }

//         pub const fn empty(ty: Type, not_null: bool) -> Self {
//             let inner = match ty {
//                 T::U8 => C::U8(Vec::new()),
//                 T::I32 => C::I32(Vec::new()),
//                 T::String => C::String(Vec::new()),
//                 T::Date => C::Date(Vec::new()),
//                 T::DateTime => C::DateTime(Vec::new()),
//             };
//             let nulls = if not_null { None } else { Some(BitVec::EMPTY) };
//             Self { inner, nulls }
//         }

//         pub fn iter(&self) -> IterTy<'_> {
//             fn _iter<'a, T>(col: &'a [MaybeUninit<T>], nulls: &'a Option<BitVec>) -> Iter<'a, T> {
//                 if let Some(nulls) = nulls {
//                     Iter::Nullable(NullableIter {
//                         index: 0,
//                         data: col,
//                         nulls,
//                     })
//                 } else {
//                     Iter::NotNull(NotNullIter {
//                         index: 0,
//                         data: unsafe { col.assume_init_ref() },
//                     })
//                 }
//             }

//             match &self.inner {
//                 C::U8(col) => IterTy::U8(_iter(col, &self.nulls)),
//                 C::I32(col) => IterTy::I32(_iter(col, &self.nulls)),
//                 C::String(col) => IterTy::String(_iter(col, &self.nulls)),
//                 C::Date(col) => IterTy::Date(_iter(col, &self.nulls)),
//                 C::DateTime(col) => IterTy::DateTime(_iter(col, &self.nulls)),
//             }
//         }
//     }

//     impl Clone for ValueColumn {
//         fn clone(&self) -> Self {
//             let nulls = self.nulls.clone();

//             fn owning<T: Clone>(
//                 vals: &[MaybeUninit<T>],
//                 nulls: &Option<BitVec>,
//             ) -> Vec<MaybeUninit<T>> {
//                 if let Some(nulls) = nulls {
//                     vals.iter()
//                         .zip(nulls.iter())
//                         .map(|(v, is_null)| {
//                             if *is_null {
//                                 MaybeUninit::uninit()
//                             } else {
//                                 MaybeUninit::new(unsafe { v.assume_init_ref() }.clone())
//                             }
//                         })
//                         .collect()
//                 } else {
//                     vals.iter()
//                         .map(|v| MaybeUninit::new(unsafe { v.assume_init_ref() }.clone()))
//                         .collect()
//                 }
//             }

//             let inner = match &self.inner {
//                 C::U8(vals) => C::U8(vals.clone()),
//                 C::I32(vals) => C::I32(vals.clone()),
//                 C::Date(vals) => C::Date(vals.clone()),
//                 C::DateTime(vals) => C::DateTime(vals.clone()),
//                 C::String(vals) => C::String(owning(vals, &nulls)),
//             };

//             Self { inner, nulls }
//         }
//     }

//     pub enum IterTy<'a> {
//         U8(Iter<'a, u8>),
//         I32(Iter<'a, i32>),
//         String(Iter<'a, String>),
//         Date(Iter<'a, NaiveDate>),
//         DateTime(Iter<'a, NaiveDateTime>),
//     }

//     #[derive(Clone, Copy, Debug)]
//     pub enum Iter<'a, T> {
//         NotNull(NotNullIter<'a, T>),
//         Nullable(NullableIter<'a, T>),
//     }

//     #[derive(Clone, Copy, Debug, Serialize)]
//     pub struct NotNullIter<'a, T> {
//         data: &'a [T],
//     }

//     #[derive(Clone, Copy, Debug, Serialize)]
//     pub struct NullableRef<'a, T> {
//         data: &'a [MaybeUninit<T>],
//         nulls: &'a BitVec,
//     }

//     impl Serialize for NullableRef<'_, T> {
//         fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//             for
//             iter.collect::<Vec<_>>().serialize(serializer)
//         }
//     }

//     impl<'a, T> Iterator for NotNullIter<'a, T> {
//         type Item = &'a T;

//         fn next(&mut self) -> Option<Self::Item> {
//             if self.index < self.data.len() {
//                 let value = &self.data[self.index];
//                 self.index += 1;
//                 Some(value)
//             } else {
//                 None
//             }
//         }
//     }

//     impl<'a, T> Iterator for NullableIter<'a, T> {
//         type Item = Option<&'a T>;

//         fn next(&mut self) -> Option<Self::Item> {
//             if self.index < self.data.len() {
//                 let is_null = self
//                     .nulls
//                     .get(self.index)
//                     .expect("must encode null for every value");
//                 let value = &self.data[self.index];
//                 self.index += 1;

//                 Some((!is_null).then_some(unsafe { value.assume_init_ref() }))
//             } else {
//                 None
//             }
//         }
//     }

//     impl<'a, T> Iterator for Iter<'a, T> {
//         type Item = Option<&'a T>;

//         fn next(&mut self) -> Option<Self::Item> {
//             match self {
//                 Iter::NotNull(iter) => iter.next().map(Some),
//                 Iter::Nullable(iter) => iter.next(),
//             }
//         }
//     }

//     impl<'a> Iterator for IterTy<'a> {
//         type Item = Option<ValueRef<'a>>;

//         fn next(&mut self) -> Option<Self::Item> {
//             match self {
//                 IterTy::U8(iter) => iter.next().map(|opt| opt.map(ValueRef::U8)),
//                 IterTy::I32(iter) => iter.next().map(|opt| opt.map(ValueRef::I32)),
//                 IterTy::String(iter) => iter.next().map(|opt| opt.map(|s| ValueRef::String(&s))),
//                 IterTy::Date(iter) => iter.next().map(|opt| opt.map(ValueRef::Date)),
//                 IterTy::DateTime(iter) => iter.next().map(|opt| opt.map(ValueRef::DateTime)),
//             }
//         }
//     }

//     mod col_serde {
//         use serde::{Deserialize, Serialize};

//         use super::*;

//         impl Serialize for ValueColumn {
//             fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//                 let iter = self.iter();

//                 serializer.serialize_tuple_variant(name, variant_index, variant, len)

//                 iter.collect::<Vec<_>>().serialize(serializer)
//             }
//         }
//     }

//     // #[derive(Clone, Debug, Default, Serialize, Deserialize)]
//     // pub struct Table {
//     //     pub columns: Vec<(String, ValueColumn)>,
//     // }
// }

// // use std::pin::pin;

// // use futures::StreamExt;
// // use leptos::prelude::*;
// // use mysql_async::consts::ColumnFlags;
// // use mysql_async::Row;
// // use serde::{Deserialize, Serialize};
// // use types::*;

// // #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// // pub enum MyTable {
// //     People,
// //     Clubs,
// //     Roles,
// //     Membership,
// //     Events,
// //     PhysicalEvents,
// //     VirtualEvents,
// //     Addresses,
// // }

// // impl MyTable {
// //     pub fn name(&self) -> &'static str {
// //         use MyTable as T;
// //         match self {
// //             T::People => "People",
// //             T::Clubs => "Clubs",
// //             T::Roles => "Roles",
// //             T::Membership => "Membership",
// //             T::Events => "Events",
// //             T::PhysicalEvents => "PhysicalEvents",
// //             T::VirtualEvents => "VirtualEvents",
// //             T::Addresses => "Addresses",
// //         }
// //     }
// // }

// // #[server]
// // pub async fn fetch(table: MyTable) -> Result<Table, ServerFnError> {
// //     use mysql_async::prelude::*;
// //     use mysql_async::Pool;

// //     let mut conn = use_context::<Pool>().expect("global").get_conn().await?;

// //     let stmt = format!("SELECT * FROM {};", table.name());
// //     let stream = conn.exec_stream(stmt, ()).await?.peekable();
// //     let stream = pin!(stream);

// //     let Some(cols) = stream.as_mut().peek().await else {
// //         return Ok(Table::default());
// //     };

// //     let row: &Row = cols.as_ref()?;
// //     let columns = row
// //         .columns()
// //         .iter()
// //         .map(|c| {
// //             let cname = c.name_str().to_string();
// //             let ctype = c.column_type();
// //             let cflags = c.flags();

// //             let not_null = cflags.contains(ColumnFlags::NOT_NULL_FLAG);

// //             use mysql_async::consts::ColumnType as CT;
// //             let value_column = match ctype {
// //                 CT::MYSQL_TYPE_TINY => ValueColumn::empty_u8(not_null),
// //                 CT::MYSQL_TYPE_LONG => ValueColumn::empty_i32(not_null),
// //                 CT::MYSQL_TYPE_VARCHAR | CT::MYSQL_TYPE_STRING | CT::MYSQL_TYPE_VAR_STRING => {
// //                     ValueColumn::empty_string(not_null)
// //                 }
// //                 CT::MYSQL_TYPE_DATE | CT::MYSQL_TYPE_NEWDATE => ValueColumn::empty_date(not_null),
// //                 CT::MYSQL_TYPE_DATETIME
// //                 | CT::MYSQL_TYPE_TIMESTAMP
// //                 | CT::MYSQL_TYPE_DATETIME2
// //                 | CT::MYSQL_TYPE_TIMESTAMP2 => ValueColumn::empty_datetime(not_null),
// //                 _ => unimplemented!("unsupported column type: {ctype:?}"),
// //             };
// //             (cname, value_column)
// //         })
// //         .collect();

// //     let table = Table { columns };

// //     // while let Some(row) = stream.next().await {
// //     //     let row = row?;
// //     //     for (i, (_, col)) in table.columns.iter_mut().enumerate() {
// //     //         col.push(row[i]);
// //     //     }
// //     // }

// //     // .exec_fold(stmt, (), Table::default(), |table, row: Row| {
// //     //     for col in &*row.columns() {
// //     //         let ctype = col.column_type();
// //     //         let cname = col.name_str();
// //     //     }

// //     //     //
// //     //     todo!("parse row into table")
// //     // })
// //     // .await?;

// //     Ok(table)
// // }
