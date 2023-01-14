pub mod graph {
    use self::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Default, PartialEq, Eq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            let mut node = self.nodes.iter().find(|x| x.inner == node_name);
            node.take()
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let attrs: HashMap<String, String> = attrs
                .iter()
                .map(|(x, y)| (x.to_string(), y.to_string()))
                .collect();

            Self { attrs, ..self }
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Eq, Debug, Default)]
            pub struct Edge {
                edge_one: String,
                edge_two: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(edge_one: &str, edge_two: &str) -> Self {
                    let edge_one = edge_one.to_owned();
                    let edge_two = edge_two.to_owned();
                    Self {
                        edge_one,
                        edge_two,
                        ..Default::default()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs: HashMap<String, String> = attrs
                        .iter()
                        .map(|(x, y)| (x.to_string(), y.to_string()))
                        .collect();

                    Self { attrs, ..self }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    let value = self.attrs.get(key).map(|x| &x[..]).take();
                    value
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub struct Node {
                pub inner: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(inner: &str) -> Self {
                    Node {
                        inner: inner.to_owned(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs: HashMap<String, String> = attrs
                        .iter()
                        .map(|(x, y)| (x.to_string(), y.to_string()))
                        .collect();

                    Self { attrs, ..self }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    let value = self.attrs.get(key).map(|x| &x[..]).take();
                    value
                }
            }
        }
    }
}
