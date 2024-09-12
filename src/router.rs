use crate::request::Request;
use crate::response::Response;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub type RequestHandler = fn(&Request) -> Result<Response, Box<dyn Error>>;

#[derive(Default)]
struct TrieNode {
    params: RwLock<HashMap<String, String>>,
    handler: RwLock<Option<RequestHandler>>,
    children: RwLock<HashMap<String, Arc<TrieNode>>>,
}

#[derive(Clone)]
pub struct Router {
    root: Arc<TrieNode>,
    params_regex: Arc<Regex>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            root: Arc::new(TrieNode::default()),
            params_regex: Arc::new(Regex::new(r":([a-z0-9_]+):").unwrap()),
        }
    }

    pub fn define_route(&self, path: String, handler: RequestHandler) {
        let mut current_node = Arc::clone(&self.root);

        for mut segment in path.split('/') {
            if segment.is_empty() {
                segment = "/";
            }

            // println!("{} -> {}", path, segment);

            let next_node = {
                let mut children = current_node.children.write().unwrap();

                children
                    .entry(segment.to_string())
                    .or_insert_with(|| Arc::new(TrieNode::default()))
                    .clone()
            };

            current_node = next_node;
        }

        let names = path.split(":").filter(|c| !c.contains("/")).collect::<Vec<&str>>();

        let mut path_params = HashMap::new();

        for (i, value) in self.params_regex.captures_iter(&path).enumerate() {
            path_params.insert(names.get(i).unwrap().to_string(), value[1].trim().to_string());
        }

        current_node.params.write().unwrap().extend(path_params);

        *current_node.handler.write().unwrap() = Some(handler);
    }

    pub fn get_handler(&self, path: &str) -> Option<RequestHandler> {
        let mut current_node = Arc::clone(&self.root);

        for mut segment in path.split('/') {
            if segment.is_empty() {
                segment = "/";
            }

            let next_node = {
                let mut children = current_node.children.read().unwrap();

                match children.get(segment) {
                    Some(route) => {
                        Some(Arc::clone(route))
                    }
                    None => { None }
                }
            };


            if let Some(r) = next_node {
                current_node = r; // Use the cloned Arc here
            } else {
                return None;
            }
        };

        let params = *current_node.params.read().unwrap();
        let handler = *current_node.handler.read().unwrap();

        Some((handler?, params?))
    }
}