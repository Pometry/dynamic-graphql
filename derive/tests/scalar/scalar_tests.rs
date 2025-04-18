use std::borrow::Cow;

use dynamic_graphql::App;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Scalar;
use dynamic_graphql::ScalarValue;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::internal::Scalar;
use dynamic_graphql::internal::TypeName;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_scalar() {
    #[derive(Scalar)]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    assert_eq!(<MyString as Scalar>::get_scalar_type_name(), "MyString");
}

#[test]
fn test_impl_scalar_with_rename() {
    #[derive(Scalar)]
    #[graphql(name = "OtherString")]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    assert_eq!(<MyString as Scalar>::get_scalar_type_name(), "OtherString");
}

#[test]
fn test_schema() {
    #[derive(Scalar)]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    scalar MyString

    type Query {
      value: MyString!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");
}

#[test]
fn test_schema_scalar_as_input() {
    #[derive(Scalar)]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: MyString) -> String {
            value.0
        }
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    scalar MyString

    type Query {
      value(value: MyString!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");
}

#[test]
fn test_schema_with_rename() {
    #[derive(Scalar)]
    #[graphql(name = "OtherString")]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    scalar OtherString

    type Query {
      value: OtherString!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");
}

#[test]
fn test_schema_with_type_name() {
    #[derive(Scalar)]
    #[graphql(get_type_name)]
    struct MyString(String);

    impl TypeName for MyString {
        fn get_type_name() -> Cow<'static, str> {
            "OtherString".into()
        }
    }

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    scalar OtherString

    type Query {
      value: OtherString!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");
}

#[test]
fn test_schema_with_doc() {
    /// this is my special string
    #[derive(Scalar)]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    "this is my special string"
    scalar MyString

    type Query {
      value: MyString!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
#[ignore]
fn test_schema_with_specified_by_url() {
    #[derive(Scalar)]
    #[graphql(specified_by_url = "https://example.com")]
    struct MyString(String);

    impl ScalarValue for MyString {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            unimplemented!()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    scalar MyString @specifiedBy(url: "https://example.com")

    type Query {
      value: MyString!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @specifiedBy(url: String!) on SCALAR

    schema {
      query: Query
    }
    "###);
}
