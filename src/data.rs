use mysql::prelude::*;
use mysql::*;

fn a() {
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )?;

    struct Payment {
        customer_id: i32,
        amount: i32,
        account_name: Option<String>,
    }

    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
          VALUES (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| {
            params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }
        }),
    )?;
    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments: Vec<Row> = conn.exec(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    let r = selected_payments[0];
    let cols = r.columns();
    let col = cols[0];
    col.
    use mysql::consts::ColumnType as CT;
    match col.column_type() {
        CT::
    }
    let v: Value = r.get(0).unwrap();

    let a = v

    // assert_eq!(payments, selected_payments);
    // println!("Yay!");
    // Ok(())
}

macro_rules! call_with {
    ($fun:tt, $args:tt, $dtype:expr) => {{
        use polars::datatypes::DataType as DT;
        match $dtype {
            DT::Boolean => $fun::<bool>($args),
            DT::UInt8 => $fun::<u8>($args),
            DT::UInt16 => $fun::<u16>($args),
            DT::UInt32 => $fun::<u32>($args),
            DT::UInt64 => $fun::<u64>($args),
            DT::Int8 => $fun::<i8>($args),
            DT::Int16 => $fun::<i16>($args),
            DT::Int32 => $fun::<i32>($args),
            DT::Int64 => $fun::<i64>($args),
            DT::Float32 => $fun::<f32>($args),
            DT::Float64 => $fun::<f64>($args),
            DT::String => $fun::<String>($args),
            DT::Binary => $fun::<Vec<u8>>($args),
            DT::Date => $fun::<chrono::NaiveDate>($args),
            DT::Datetime(_, _) => $fun::<chrono::NaiveDateTime>($args),
            DT::Duration(_) => $fun::<chrono::Duration>($args),
            DT::Time => $fun::<chrono::NaiveTime>($args),
            _ => panic!(),
        }
    }};
}

#[server]
async fn select() -> Result<DataFrame, ServerFnError> {
    use sqlx::{query, MySql, Pool};

    let mut conn = use_context::<Pool<MySql>>()
        .expect("global context")
        .acquire()
        .await
        .unwrap();

    let mut stream = query("SELECT * FROM People;").fetch(&mut *conn).peekable();
    let mut stream = Pin::new(&mut stream);

    let Some(row) = stream.as_mut().peek().await else {
        return Ok(DataFrame::empty());
    };
    // TODO proper error handling
    let row = row.as_ref().unwrap();

    let mut columns: Vec<(String, DataType, AnyVec)> = row
        .columns()
        .iter()
        .map(|c| {
            let name = c.name();
            let dtype = type_sqlx_to_polars(c.type_info());
            fn new<T: 'static>(_: ()) -> AnyVec {
                AnyVec::new::<T>()
            }

            let data = call_with!(new, (), dtype);
            (name.to_owned(), dtype, data)
        })
        .collect();

    while let Some(row) = stream.next().await {
        let row = row?;

        for (ordinal, (name, dtype, data)) in columns.iter_mut().enumerate() {
            fn fun<'r, T: 'static + sqlx::Decode<'r, sqlx::MySql> + sqlx::Type<sqlx::MySql>>(
                (data, row, ordinal): (&mut AnyVec, &'r MySqlRow, usize),
            ) {
                let mut col = data.downcast_mut::<T>().unwrap();
                col.push(row.get(ordinal));
            }

            call_with!(fun, (data, &row, ordinal), dtype);
        }
    }

    let columns = columns
        .into_iter()
        .map(|(name, dtype, data)| {
            fn col<T: 'static>((name, mut data): (String, AnyVec)) -> Column
            where
                Series: NamedFrom<Vec<T>, [T]>,
            {
                let data = data
                    .downcast_mut::<T>()
                    .unwrap()
                    .drain(..)
                    .collect::<Vec<T>>();
                Column::new(name.into(), data)
            }
            call_with!(col, (name, data), dtype)
        })
        .collect();

    let df = DataFrame::new(0, columns).unwrap();

    Ok(df)
}

fn type_sqlx_to_polars(ty: &impl TypeInfo) -> DataType {
    use polars::datatypes::DataType as DT;

    // TODO find a library that makes this better
    match ty.name() {
        "BOOLEAN" => DT::Boolean,
        "TINYINT UNSIGNED" => DT::UInt8,
        "SMALLINT UNSIGNED" => DT::UInt16,
        "INT UNSIGNED" => DT::UInt32,
        "MEDIUMINT UNSIGNED" => DT::UInt32, // UInt24
        "BIGINT UNSIGNED" => DT::UInt64,
        "TINYINT" => DT::Int8,
        "SMALLINT" => DT::Int16,
        "INT" => DT::Int32,
        "MEDIUMINT" => DT::Int32, // Int24
        "BIGINT" => DT::Int64,
        "FLOAT" => DT::Float32,
        "DOUBLE" => DT::Float64,
        "NULL" => DT::Null,
        "TIMESTAMP" => DT::Datetime(TimeUnit::Microseconds, Some(TimeZone::UTC)),
        "DATE" => DT::Date,
        "TIME" => DT::Time,
        "DATETIME" => DT::Datetime(TimeUnit::Microseconds, None),
        "YEAR" => DT::Int16,
        "BIT" => DT::UInt64,
        "ENUM" => DT::Unknown(UnknownKind::Str),
        "SET" => DT::Unknown(UnknownKind::Any),
        "DECIMAL" => DT::Unknown(UnknownKind::Any),
        "GEOMETRY" => DT::Unknown(UnknownKind::Any),
        "JSON" => DT::Unknown(UnknownKind::Str),
        "BINARY" => DT::Binary,
        "VARBINARY" => DT::Binary,
        "CHAR" => DT::String,
        "VARCHAR" => DT::String,
        "TINYBLOB" => DT::Binary,
        "TINYTEXT" => DT::String,
        "BLOB" => DT::Binary,
        "TEXT" => DT::String,
        "MEDIUMBLOB" => DT::Binary,
        "MEDIUMTEXT" => DT::String,
        "LONGBLOB" => DT::Binary,
        "LONGTEXT" => DT::String,
        _ => DT::Unknown(UnknownKind::Any),
    }
}
