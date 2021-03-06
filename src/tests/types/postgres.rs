use crate::tests::test_api::PostgreSql;
use std::str::FromStr;

test_type!(boolean(
    PostgreSql,
    "boolean",
    Value::Boolean(None),
    Value::boolean(true),
    Value::boolean(false),
));

#[cfg(feature = "array")]
test_type!(boolean_array(
    PostgreSql,
    "boolean[]",
    Value::Array(None),
    Value::array(vec![true, false, true]),
));

test_type!(int2(
    PostgreSql,
    "int2",
    Value::Integer(None),
    Value::integer(i16::MIN),
    Value::integer(i16::MAX),
));

#[cfg(feature = "array")]
test_type!(int2_array(
    PostgreSql,
    "int2[]",
    Value::Array(None),
    Value::array(vec![1, 2, 3]),
));

test_type!(int4(
    PostgreSql,
    "int4",
    Value::Integer(None),
    Value::integer(i32::MIN),
    Value::integer(i32::MAX),
));

#[cfg(feature = "array")]
test_type!(int4_array(
    PostgreSql,
    "int4[]",
    Value::Array(None),
    Value::array(vec![1, 2, 3]),
));

test_type!(int8(
    PostgreSql,
    "int8",
    Value::Integer(None),
    Value::integer(i64::MIN),
    Value::integer(i64::MAX),
));

#[cfg(feature = "array")]
test_type!(int8_array(
    PostgreSql,
    "int8[]",
    Value::Array(None),
    Value::array(vec![1, 2, 3]),
));

test_type!(oid(PostgreSql, "oid", Value::Integer(None), Value::integer(10000)));

#[cfg(feature = "array")]
test_type!(oid_array(
    PostgreSql,
    "oid[]",
    Value::Array(None),
    Value::array(vec![1, 2, 3]),
));

test_type!(serial2(
    PostgreSql,
    "serial2",
    Value::integer(i16::MIN),
    Value::integer(i16::MAX),
));

test_type!(serial4(
    PostgreSql,
    "serial4",
    Value::integer(i32::MIN),
    Value::integer(i32::MAX),
));

test_type!(serial8(
    PostgreSql,
    "serial8",
    Value::integer(i64::MIN),
    Value::integer(i64::MAX),
));

test_type!(decimal(
    PostgreSql,
    "decimal(10,2)",
    Value::Real(None),
    Value::real(rust_decimal::Decimal::new(314, 2))
));

#[cfg(feature = "array")]
test_type!(decimal_array(
    PostgreSql,
    "decimal(10,2)[]",
    Value::Array(None),
    Value::array(vec![
        rust_decimal::Decimal::new(314, 2),
        rust_decimal::Decimal::new(512, 2)
    ])
));

test_type!(float4(
    PostgreSql,
    "float4",
    Value::Real(None),
    Value::real(rust_decimal::Decimal::from_str("1.1234").unwrap())
));

#[cfg(feature = "array")]
test_type!(float4_array(
    PostgreSql,
    "float4[]",
    Value::Array(None),
    Value::array(vec![
        rust_decimal::Decimal::from_str("1.1234").unwrap(),
        rust_decimal::Decimal::from_str("4.3210").unwrap(),
    ])
));

test_type!(float8(
    PostgreSql,
    "float8",
    Value::Real(None),
    Value::real(rust_decimal::Decimal::from_str("1.12345").unwrap())
));

#[cfg(feature = "array")]
test_type!(float8_array(
    PostgreSql,
    "float8[]",
    Value::Array(None),
    Value::array(vec![
        rust_decimal::Decimal::from_str("1.1234").unwrap(),
        rust_decimal::Decimal::from_str("4.3210").unwrap(),
    ])
));

test_type!(money(
    PostgreSql,
    "money",
    Value::Real(None),
    Value::real(rust_decimal::Decimal::from_str("1.12").unwrap())
));

#[cfg(feature = "array")]
test_type!(money_array(
    PostgreSql,
    "money[]",
    Value::Array(None),
    Value::array(vec![
        rust_decimal::Decimal::from_str("1.12").unwrap(),
        rust_decimal::Decimal::from_str("1.12").unwrap()
    ])
));

test_type!(char(PostgreSql, "char(6)", Value::Text(None), Value::text("foobar")));

#[cfg(feature = "array")]
test_type!(char_array(
    PostgreSql,
    "char(6)[]",
    Value::Array(None),
    Value::array(vec![Value::text("foobar"), Value::text("omgwtf")])
));

test_type!(varchar(
    PostgreSql,
    "varchar(255)",
    Value::Text(None),
    Value::text("foobar")
));

#[cfg(feature = "array")]
test_type!(varchar_array(
    PostgreSql,
    "varchar(255)[]",
    Value::Array(None),
    Value::array(vec![Value::text("foobar"), Value::text("omgwtf")])
));

test_type!(text(PostgreSql, "text", Value::Text(None), Value::text("foobar")));

#[cfg(feature = "array")]
test_type!(text_array(
    PostgreSql,
    "text[]",
    Value::Array(None),
    Value::array(vec![Value::text("foobar"), Value::text("omgwtf")])
));

test_type!(bit(PostgreSql, "bit(4)", Value::Text(None), Value::text("1001")));

#[cfg(feature = "array")]
test_type!(bit_array(
    PostgreSql,
    "bit(4)[]",
    Value::Array(None),
    Value::array(vec![Value::text("1001"), Value::text("0110")])
));

test_type!(varbit(
    PostgreSql,
    "varbit(20)",
    Value::Text(None),
    Value::text("001010101")
));

#[cfg(feature = "array")]
test_type!(varbit_array(
    PostgreSql,
    "varbit(20)[]",
    Value::Array(None),
    Value::array(vec![Value::text("001010101"), Value::text("01101111")])
));

test_type!(inet(PostgreSql, "inet", Value::Text(None), Value::text("127.0.0.1")));

#[cfg(feature = "array")]
test_type!(inet_array(
    PostgreSql,
    "inet[]",
    Value::Array(None),
    Value::array(vec![Value::text("127.0.0.1"), Value::text("192.168.1.1")])
));

#[cfg(feature = "json-1")]
test_type!(json(
    PostgreSql,
    "json",
    Value::Json(None),
    Value::json(serde_json::json!({"foo": "bar"}))
));

#[cfg(all(feature = "json-1", feature = "array"))]
test_type!(json_array(
    PostgreSql,
    "json[]",
    Value::Array(None),
    Value::array(vec![
        serde_json::json!({"foo": "bar"}),
        serde_json::json!({"omg": false})
    ])
));

#[cfg(feature = "json-1")]
test_type!(jsonb(
    PostgreSql,
    "jsonb",
    Value::Json(None),
    Value::json(serde_json::json!({"foo": "bar"}))
));

#[cfg(all(feature = "json-1", feature = "array"))]
test_type!(jsonb_array(
    PostgreSql,
    "jsonb[]",
    Value::Array(None),
    Value::array(vec![
        serde_json::json!({"foo": "bar"}),
        serde_json::json!({"omg": false})
    ])
));

#[cfg(feature = "uuid-0_8")]
test_type!(uuid(
    PostgreSql,
    "uuid",
    Value::Uuid(None),
    Value::uuid(uuid::Uuid::from_str("936DA01F-9ABD-4D9D-80C7-02AF85C822A8").unwrap())
));

#[cfg(feature = "chrono-0_4")]
test_type!(date(
    PostgreSql,
    "date",
    Value::Date(None),
    Value::date(chrono::NaiveDate::from_ymd(2020, 4, 20))
));

#[cfg(all(feature = "chrono-0_4", feature = "array"))]
test_type!(date_array(
    PostgreSql,
    "date[]",
    Value::Array(None),
    Value::array(vec![chrono::NaiveDate::from_ymd(2020, 4, 20)])
));

#[cfg(feature = "chrono-0_4")]
test_type!(time(
    PostgreSql,
    "time",
    Value::Time(None),
    Value::time(chrono::NaiveTime::from_hms(16, 20, 00))
));

#[cfg(all(feature = "chrono-0_4", feature = "array"))]
test_type!(time_array(
    PostgreSql,
    "time[]",
    Value::Array(None),
    Value::array(vec![chrono::NaiveTime::from_hms(16, 20, 00)])
));

#[cfg(feature = "chrono-0_4")]
test_type!(timestamp(PostgreSql, "timestamp", Value::DateTime(None), {
    let dt = chrono::DateTime::parse_from_rfc3339("2020-02-27T19:10:22Z").unwrap();
    Value::datetime(dt.with_timezone(&chrono::Utc))
}));

#[cfg(all(feature = "chrono-0_4", feature = "array"))]
test_type!(timestamp_array(PostgreSql, "timestamp[]", Value::Array(None), {
    let dt = chrono::DateTime::parse_from_rfc3339("2020-02-27T19:10:22Z").unwrap();
    Value::array(vec![dt.with_timezone(&chrono::Utc)])
}));

#[cfg(feature = "chrono-0_4")]
test_type!(timestamptz(PostgreSql, "timestamptz", Value::DateTime(None), {
    let dt = chrono::DateTime::parse_from_rfc3339("2020-02-27T19:10:22Z").unwrap();
    Value::datetime(dt.with_timezone(&chrono::Utc))
}));

#[cfg(all(feature = "chrono-0_4", feature = "array"))]
test_type!(timestamptz_array(PostgreSql, "timestamptz[]", Value::Array(None), {
    let dt = chrono::DateTime::parse_from_rfc3339("2020-02-27T19:10:22Z").unwrap();
    Value::array(vec![dt.with_timezone(&chrono::Utc)])
}));

test_type!(bytea(
    PostgreSql,
    "bytea",
    Value::Bytes(None),
    Value::bytes(b"DEADBEEF".to_vec())
));

#[cfg(feature = "array")]
test_type!(bytea_array(
    PostgreSql,
    "bytea[]",
    Value::Array(None),
    Value::array(vec![
        Value::bytes(b"DEADBEEF".to_vec()),
        Value::bytes(b"BEEFBEEF".to_vec())
    ])
));

/* Reserved for SQLx. All of these are broken in the current impl!
#[cfg(feature = "chrono-0_4")]
test_type!(timetz(PostgreSql, "timetz", {
    let dt = chrono::DateTime::parse_from_rfc3339("1970-01-01T19:10:22Z").unwrap();
    Value::time(chrono::NaiveTime::from_hms(19, 10, 22))
}));

#[cfg(all(feature = "chrono-0_4", feature = "array"))]
test_type!(timetz_array(PostgreSql, "timetz[]", {
    let dt = chrono::DateTime::parse_from_rfc3339("1970-01-01T19:10:22Z").unwrap();
    Value::array(vec![dt.with_timezone(&chrono::Utc)])
}));

test_type!(cidr(PostgreSql, "cidr", Value::text("0.0.0.0/0")));

#[cfg(feature = "array")]
test_type!(cidr_array(
    PostgreSql,
    "cidr[]",
    Value::array(vec![Value::text("127.0.0.1/16"), Value::text("192.168.1.1/24")])
));



#[cfg(all(feature = "uuid-0_8", feature = "array"))]
test_type!(uuid_array(
    PostgreSql,
    "uuid[]",
    Value::array(vec![
        uuid::Uuid::from_str("936DA01F-9ABD-4D9D-80C7-02AF85C822A8").unwrap()
    ])
));

*/
