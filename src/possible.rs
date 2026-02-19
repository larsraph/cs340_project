mod types {
    use std::mem::MaybeUninit;

    use bitvec::prelude::*;
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::de::VariantAccess;
    use serde::ser::{SerializeSeq, SerializeStruct, SerializeTupleVariant};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
    enum Type {
        U8,
        I32,
        Date,
        DateTime,
        String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Value {
        U8(u8),
        I32(i32),
        String(String),
        Date(NaiveDate),
        DateTime(NaiveDateTime),
    }

    #[derive(Debug)]
    pub enum ValueColumnInner {
        U8(Vec<MaybeUninit<u8>>),
        I32(Vec<MaybeUninit<i32>>),
        String(Vec<MaybeUninit<String>>),
        Date(Vec<MaybeUninit<NaiveDate>>),
        DateTime(Vec<MaybeUninit<NaiveDateTime>>),
    }

    impl ValueColumnInner {
        pub fn push_uninit(&mut self) {
            match self {
                ValueColumnInner::U8(col) => col.push(MaybeUninit::uninit()),
                ValueColumnInner::I32(col) => col.push(MaybeUninit::uninit()),
                ValueColumnInner::String(col) => col.push(MaybeUninit::uninit()),
                ValueColumnInner::Date(col) => col.push(MaybeUninit::uninit()),
                ValueColumnInner::DateTime(col) => col.push(MaybeUninit::uninit()),
            }
        }

        pub fn len(&self) -> usize {
            match self {
                ValueColumnInner::U8(col) => col.len(),
                ValueColumnInner::I32(col) => col.len(),
                ValueColumnInner::String(col) => col.len(),
                ValueColumnInner::Date(col) => col.len(),
                ValueColumnInner::DateTime(col) => col.len(),
            }
        }
    }

    #[derive(Debug)]
    pub struct ValueColumn {
        pub values: ValueColumnInner,
        pub nulls: Option<BitVec>,
    }

    impl ValueColumn {
        pub fn push(&mut self, value: Option<Value>) {
            match (value, &mut self.values, &mut self.nulls) {
                (Some(Value::U8(v)), ValueColumnInner::U8(col), nulls) => {
                    col.push(MaybeUninit::new(v));
                    nulls.as_mut().map(|nulls| nulls.push(false));
                }
                (Some(Value::I32(v)), ValueColumnInner::I32(col), nulls) => {
                    col.push(MaybeUninit::new(v));
                    nulls.as_mut().map(|nulls| nulls.push(false));
                }
                (Some(Value::String(v)), ValueColumnInner::String(col), nulls) => {
                    col.push(MaybeUninit::new(v));
                    nulls.as_mut().map(|nulls| nulls.push(false));
                }
                (Some(Value::Date(v)), ValueColumnInner::Date(col), nulls) => {
                    col.push(MaybeUninit::new(v));
                    nulls.as_mut().map(|nulls| nulls.push(false));
                }
                (Some(Value::DateTime(v)), ValueColumnInner::DateTime(col), nulls) => {
                    col.push(MaybeUninit::new(v));
                    nulls.as_mut().map(|nulls| nulls.push(false));
                }
                (None, col, Some(nulls)) => {
                    col.push_uninit();
                    nulls.push(true);
                }
                _ => panic!("mismatched value type"),
            }
        }

        pub fn empty_u8(not_null: bool) -> Self {
            ValueColumn {
                values: ValueColumnInner::U8(Vec::new()),
                nulls: not_null.then(BitVec::new),
            }
        }

        pub fn empty_i32(not_null: bool) -> Self {
            ValueColumn {
                values: ValueColumnInner::I32(Vec::new()),
                nulls: not_null.then(BitVec::new),
            }
        }

        pub fn empty_string(not_null: bool) -> Self {
            ValueColumn {
                values: ValueColumnInner::String(Vec::new()),
                nulls: not_null.then(BitVec::new),
            }
        }

        pub fn empty_date(not_null: bool) -> Self {
            ValueColumn {
                values: ValueColumnInner::Date(Vec::new()),
                nulls: not_null.then(BitVec::new),
            }
        }

        pub fn empty_datetime(not_null: bool) -> Self {
            ValueColumn {
                values: ValueColumnInner::DateTime(Vec::new()),
                nulls: not_null.then(BitVec::new),
            }
        }
    }

    impl Clone for ValueColumn {
        fn clone(&self) -> Self {
            match &self.values {
                ValueColumnInner::U8(vals) => Self {
                    values: ValueColumnInner::U8(vals.clone()),
                    nulls: self.nulls.clone(),
                },
                ValueColumnInner::I32(vals) => Self {
                    values: ValueColumnInner::I32(vals.clone()),
                    nulls: self.nulls.clone(),
                },
                ValueColumnInner::Date(vals) => Self {
                    values: ValueColumnInner::Date(vals.clone()),
                    nulls: self.nulls.clone(),
                },
                ValueColumnInner::DateTime(vals) => Self {
                    values: ValueColumnInner::DateTime(vals.clone()),
                    nulls: self.nulls.clone(),
                },
                ValueColumnInner::String(vals) => {
                    if let Some(nulls) = &self.nulls {
                        return ValueColumn {
                            values: ValueColumnInner::String(
                                vals.iter()
                                    .zip(nulls.iter())
                                    .map(|(v, is_null)| {
                                        if *is_null {
                                            MaybeUninit::uninit()
                                        } else {
                                            unsafe { MaybeUninit::new(v.assume_init_ref().clone()) }
                                        }
                                    })
                                    .collect(),
                            ),
                            nulls: Some(nulls.clone()),
                        };
                    } else {
                        return ValueColumn {
                            values: ValueColumnInner::String(
                                vals.iter()
                                    .map(|v| unsafe {
                                        MaybeUninit::new(v.assume_init_ref().clone())
                                    })
                                    .collect(),
                            ),
                            nulls: None,
                        };
                    }
                }
            }
        }
    }

    impl Serialize for ValueColumn {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            struct Values<'a>(&'a ValueColumn);

            impl<'a> Serialize for Values<'a> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let mut seq = serializer.serialize_seq(Some(self.0.values.len()))?;

                    if let Some(nulls) = &self.0.nulls {
                        match &self.0.values {
                            ValueColumnInner::U8(vals) => {
                                for (mu, is_null) in vals.iter().zip(nulls.iter()) {
                                    if *is_null {
                                        seq.serialize_element(&None::<u8>)?;
                                    } else {
                                        seq.serialize_element(&Some(unsafe {
                                            mu.assume_init_ref()
                                        }))?;
                                    }
                                }
                            }
                            ValueColumnInner::I32(vals) => {
                                for (mu, is_null) in vals.iter().zip(nulls.iter()) {
                                    if *is_null {
                                        seq.serialize_element(&None::<i32>)?;
                                    } else {
                                        seq.serialize_element(&Some(unsafe {
                                            mu.assume_init_ref()
                                        }))?;
                                    }
                                }
                            }
                            ValueColumnInner::Date(vals) => {
                                for (mu, is_null) in vals.iter().zip(nulls.iter()) {
                                    if *is_null {
                                        seq.serialize_element(&None::<NaiveDate>)?;
                                    } else {
                                        seq.serialize_element(&Some(unsafe {
                                            mu.assume_init_ref()
                                        }))?;
                                    }
                                }
                            }
                            ValueColumnInner::DateTime(vals) => {
                                for (mu, is_null) in vals.iter().zip(nulls.iter()) {
                                    if *is_null {
                                        seq.serialize_element(&None::<NaiveDateTime>)?;
                                    } else {
                                        seq.serialize_element(&Some(unsafe {
                                            mu.assume_init_ref()
                                        }))?;
                                    }
                                }
                            }
                            ValueColumnInner::String(vals) => {
                                for (mu, is_null) in vals.iter().zip(nulls.iter()) {
                                    if *is_null {
                                        seq.serialize_element(&None::<String>)?;
                                    } else {
                                        seq.serialize_element(&Some(unsafe {
                                            mu.assume_init_ref()
                                        }))?;
                                    }
                                }
                            }
                        }
                    } else {
                        match &self.0.values {
                            ValueColumnInner::U8(vals) => {
                                for mu in vals.iter() {
                                    seq.serialize_element(unsafe { mu.assume_init_ref() })?;
                                }
                            }
                            ValueColumnInner::I32(vals) => {
                                for mu in vals.iter() {
                                    seq.serialize_element(unsafe { mu.assume_init_ref() })?;
                                }
                            }
                            ValueColumnInner::Date(vals) => {
                                for mu in vals.iter() {
                                    seq.serialize_element(unsafe { mu.assume_init_ref() })?;
                                }
                            }
                            ValueColumnInner::DateTime(vals) => {
                                for mu in vals.iter() {
                                    seq.serialize_element(unsafe { mu.assume_init_ref() })?;
                                }
                            }
                            ValueColumnInner::String(vals) => {
                                for mu in vals.iter() {
                                    seq.serialize_element(unsafe { mu.assume_init_ref() })?;
                                }
                            }
                        }
                    }

                    seq.end()
                }
            }

            let mut st = match &self.values {
                ValueColumnInner::U8(_) => {
                    serializer.serialize_tuple_variant("ValueColumn", 0, "U8", 1)?
                }
                ValueColumnInner::I32(_) => {
                    serializer.serialize_tuple_variant("ValueColumn", 1, "I32", 1)?
                }
                ValueColumnInner::Date(_) => {
                    serializer.serialize_tuple_variant("ValueColumn", 2, "Date", 1)?
                }
                ValueColumnInner::DateTime(_) => {
                    serializer.serialize_tuple_variant("ValueColumn", 3, "DateTime", 1)?
                }
                ValueColumnInner::String(_) => {
                    serializer.serialize_tuple_variant("ValueColumn", 4, "String", 1)?
                }
            };

            st.serialize_field(&Values(self))?;

            st.end()
        }
    }

    impl<'de> Deserialize<'de> for ValueColumn {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::{self, SeqAccess, Visitor};

            struct ValueColumnVisitor;

            impl<'de> Visitor<'de> for ValueColumnVisitor {
                type Value = ValueColumn;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("enum ValueColumn")
                }

                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: de::EnumAccess<'de>,
                {
                    let (ty, variant_access) = data.variant()?;

                    let a = variant_access.tuple_variant(1, MaybeNullSeqVisitor::<u8>::new())?;

                    match ty {
                        Type::U8 => {
                            let values: Vec<Option<u8>> = variant_access
                                .tuple_variant(1, MaybeNullSeqVisitor::<u8>::new())?;
                            let has_nulls = values.iter().any(|v| v.is_none());
                            let nulls = if has_nulls {
                                Some(values.iter().map(|v| v.is_none()).collect())
                            } else {
                                None
                            };
                            let vec = values
                                .into_iter()
                                .map(|v| match v {
                                    Some(val) => MaybeUninit::new(val),
                                    None => MaybeUninit::uninit(),
                                })
                                .collect();
                            Ok(ValueColumn {
                                values: ValueColumnInner::U8(vec),
                                nulls,
                            })
                        }
                        Type::I32 => {
                            let values: Vec<Option<i32>> = variant_access
                                .tuple_variant(1, MaybeNullSeqVisitor::<i32>::new())?;
                            let has_nulls = values.iter().any(|v| v.is_none());
                            let nulls = if has_nulls {
                                Some(values.iter().map(|v| v.is_none()).collect())
                            } else {
                                None
                            };
                            let vec = values
                                .into_iter()
                                .map(|v| match v {
                                    Some(val) => MaybeUninit::new(val),
                                    None => MaybeUninit::uninit(),
                                })
                                .collect();
                            Ok(ValueColumn {
                                values: ValueColumnInner::I32(vec),
                                nulls,
                            })
                        }
                        Type::Date => {
                            let values: Vec<Option<NaiveDate>> = variant_access
                                .tuple_variant(1, MaybeNullSeqVisitor::<NaiveDate>::new())?;
                            let has_nulls = values.iter().any(|v| v.is_none());
                            let nulls = if has_nulls {
                                Some(values.iter().map(|v| v.is_none()).collect())
                            } else {
                                None
                            };
                            let vec = values
                                .into_iter()
                                .map(|v| match v {
                                    Some(val) => MaybeUninit::new(val),
                                    None => MaybeUninit::uninit(),
                                })
                                .collect();
                            Ok(ValueColumn {
                                values: ValueColumnInner::Date(vec),
                                nulls,
                            })
                        }
                        Type::DateTime => {
                            let values: Vec<Option<NaiveDateTime>> = variant_access
                                .tuple_variant(1, MaybeNullSeqVisitor::<NaiveDateTime>::new())?;
                            let has_nulls = values.iter().any(|v| v.is_none());
                            let nulls = if has_nulls {
                                Some(values.iter().map(|v| v.is_none()).collect())
                            } else {
                                None
                            };
                            let vec = values
                                .into_iter()
                                .map(|v| match v {
                                    Some(val) => MaybeUninit::new(val),
                                    None => MaybeUninit::uninit(),
                                })
                                .collect();
                            Ok(ValueColumn {
                                values: ValueColumnInner::DateTime(vec),
                                nulls,
                            })
                        }
                        Type::String => {
                            let values: Vec<Option<String>> = variant_access
                                .tuple_variant(1, MaybeNullSeqVisitor::<String>::new())?;
                            let has_nulls = values.iter().any(|v| v.is_none());
                            let nulls = if has_nulls {
                                Some(values.iter().map(|v| v.is_none()).collect())
                            } else {
                                None
                            };
                            let vec = values
                                .into_iter()
                                .map(|v| match v {
                                    Some(val) => MaybeUninit::new(val),
                                    None => MaybeUninit::uninit(),
                                })
                                .collect();
                            Ok(ValueColumn {
                                values: ValueColumnInner::String(vec),
                                nulls,
                            })
                        }
                    }
                }
            }

            enum MaybeNullSeq<T> {
                NotNull(Vec<T>),
                Null(Vec<Option<T>>),
            }

            struct MaybeNullSeqVisitor<T>(std::marker::PhantomData<T>);

            impl<T> MaybeNullSeqVisitor<T> {
                fn new() -> Self {
                    MaybeNullSeqVisitor(std::marker::PhantomData)
                }
            }

            impl<'de, T> Visitor<'de> for MaybeNullSeqVisitor<T>
            where
                T: Deserialize<'de>,
            {
                type Value = MaybeNullSeq<T>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    seq.next_element();
                    while let Some(value) = seq.next_element()? {
                        values.push(value);
                    }
                    Ok(MaybeNullableSeq::NotNullable(values))
                }
            }

            const VARIANTS: &[&str] = &["U8", "I32", "Date", "DateTime", "String"];
            deserializer.deserialize_enum("ValueColumn", VARIANTS, ValueColumnVisitor)
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Table {
        pub columns: Vec<(String, ValueColumn)>,
    }
}

use std::pin::pin;

use futures::StreamExt;
use leptos::prelude::*;
use mysql_async::consts::ColumnFlags;
use mysql_async::Row;
use serde::{Deserialize, Serialize};
use types::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MyTable {
    People,
    Clubs,
    Roles,
    Membership,
    Events,
    PhysicalEvents,
    VirtualEvents,
    Addresses,
}

impl MyTable {
    pub fn name(&self) -> &'static str {
        use MyTable as T;
        match self {
            T::People => "People",
            T::Clubs => "Clubs",
            T::Roles => "Roles",
            T::Membership => "Membership",
            T::Events => "Events",
            T::PhysicalEvents => "PhysicalEvents",
            T::VirtualEvents => "VirtualEvents",
            T::Addresses => "Addresses",
        }
    }
}

#[server]
pub async fn fetch(table: MyTable) -> Result<Table, ServerFnError> {
    use mysql_async::prelude::*;
    use mysql_async::Pool;

    let mut conn = use_context::<Pool>().expect("global").get_conn().await?;

    let stmt = format!("SELECT * FROM {};", table.name());
    let stream = conn.exec_stream(stmt, ()).await?.peekable();
    let stream = pin!(stream);

    let Some(cols) = stream.as_mut().peek().await else {
        return Ok(Table::default());
    };

    let row: &Row = cols.as_ref()?;
    let columns = row
        .columns()
        .iter()
        .map(|c| {
            let cname = c.name_str().to_string();
            let ctype = c.column_type();
            let cflags = c.flags();

            let not_null = cflags.contains(ColumnFlags::NOT_NULL_FLAG);

            use mysql_async::consts::ColumnType as CT;
            let value_column = match ctype {
                CT::MYSQL_TYPE_TINY => ValueColumn::empty_u8(not_null),
                CT::MYSQL_TYPE_LONG => ValueColumn::empty_i32(not_null),
                CT::MYSQL_TYPE_VARCHAR | CT::MYSQL_TYPE_STRING | CT::MYSQL_TYPE_VAR_STRING => {
                    ValueColumn::empty_string(not_null)
                }
                CT::MYSQL_TYPE_DATE | CT::MYSQL_TYPE_NEWDATE => ValueColumn::empty_date(not_null),
                CT::MYSQL_TYPE_DATETIME
                | CT::MYSQL_TYPE_TIMESTAMP
                | CT::MYSQL_TYPE_DATETIME2
                | CT::MYSQL_TYPE_TIMESTAMP2 => ValueColumn::empty_datetime(not_null),
                _ => unimplemented!("unsupported column type: {ctype:?}"),
            };
            (cname, value_column)
        })
        .collect();

    let table = Table { columns };

    // while let Some(row) = stream.next().await {
    //     let row = row?;
    //     for (i, (_, col)) in table.columns.iter_mut().enumerate() {
    //         col.push(row[i]);
    //     }
    // }

    // .exec_fold(stmt, (), Table::default(), |table, row: Row| {
    //     for col in &*row.columns() {
    //         let ctype = col.column_type();
    //         let cname = col.name_str();
    //     }

    //     //
    //     todo!("parse row into table")
    // })
    // .await?;

    Ok(table)
}
