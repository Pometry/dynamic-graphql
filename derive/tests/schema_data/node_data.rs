use dynamic_graphql::App;

use self::node::NodeQuery;
use self::query::Query;
use crate::schema_utils::normalize_schema;

mod node {
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use dynamic_graphql::Context;
    use dynamic_graphql::ExpandObject;
    use dynamic_graphql::ExpandObjectFields;
    use dynamic_graphql::Instance;
    use dynamic_graphql::Interface;
    use dynamic_graphql::experimental::GetSchemaData;

    use super::query::Query;

    pub type GetNode = fn(&str) -> Option<Instance<'static, dyn Node>>;
    #[derive(Default)]
    pub struct NodeData(HashMap<String, GetNode>);
    impl Deref for NodeData {
        type Target = HashMap<String, GetNode>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for NodeData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    fn format_id<T: Node>(node: &T) -> String {
        format!("{}:{}", T::get_name(), node.get_id())
    }

    fn parse_id(id: &str) -> Option<(String, String)> {
        let mut split = id.split(':');
        let name = split.next()?;
        let id = split.next()?;
        Some((name.to_string(), id.to_string()))
    }

    #[Interface]
    pub trait Node {
        fn id(&self) -> String
        where
            Self: Sized,
        {
            format_id(self)
        }
        #[graphql(skip)]
        fn get_id(&self) -> String;
        #[graphql(skip)]
        fn get_name() -> &'static str
        where
            Self: Sized;
    }

    #[derive(ExpandObject)]
    pub struct NodeQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl NodeQuery<'_> {
        fn node(ctx: &Context<'_>, id: String) -> Option<Instance<'static, dyn Node>> {
            let (name, id) = parse_id(&id)?;
            let node_data = ctx.get_schema_data().get::<NodeData>()?;
            let get_node = node_data.0.get(&name)?;
            get_node(&id)
        }
    }
}

mod foo {
    use dynamic_graphql::Instance;
    use dynamic_graphql::SimpleObject;
    use dynamic_graphql::internal::Object;
    use dynamic_graphql::internal::Register;
    use dynamic_graphql::internal::Registry;

    use super::node::Node;
    use super::node::NodeData;

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    #[graphql(register(RegisterFooNode))]
    pub struct FooNode {
        #[graphql(skip)]
        id: String,
        name: String,
    }

    impl Node for FooNode {
        fn get_id(&self) -> String {
            self.id.to_string()
        }
        fn get_name() -> &'static str {
            "FooNode"
        }
    }

    struct RegisterFooNode;
    impl Register for RegisterFooNode {
        fn register(mut registry: Registry) -> Registry {
            let node_data: &mut NodeData = registry.data.get_mut_or_default();
            node_data.insert(
                <FooNode as Object>::get_object_type_name().to_string(),
                |id| {
                    Some(Instance::new_owned(FooNode {
                        id: id.to_string(),
                        name: "foo".to_string(),
                    }))
                },
            );
            registry
        }
    }
}

mod query {
    use dynamic_graphql::SimpleObject;

    #[derive(SimpleObject)]
    #[graphql(root)]
    pub struct Query;
}

#[derive(App)]
struct App(Query, NodeQuery<'static>, foo::FooNode);

#[tokio::test]
async fn test() {
    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type FooNode implements Node {
      name: String!
      id: String!
    }

    interface Node {
      id: String!
    }

    type Query {
      node(id: String!): Node
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            node(id: "FooNode:1") {
                id
                ... on FooNode {
                    name
                }
            }
        }
    "#;
    let res = schema.execute(query).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "node": {
                "id": "FooNode:1",
                "name": "foo",
            }
        })
    );
}
