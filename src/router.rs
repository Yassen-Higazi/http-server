use crate::request::Request;
use crate::response::Response;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub type RequestHandler = fn(&Request) -> Result<Response, Box<dyn Error>>;

#[derive(Default)]
struct TrieNode {
    params: RwLock<HashMap<String, i32>>,
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
            params_regex: Arc::new(Regex::new(r":([a-z0-9_]+)").unwrap()),
        }
    }

    pub fn define_route(&self, mut path: String, handler: RequestHandler) {
        let mut current_node = Arc::clone(&self.root);

        let mut path_params = HashMap::new();

        for capture in self.params_regex.captures_iter(&path) {
            match capture.get(1) {
                None => {}
                Some(cap) => {
                    path_params.insert(cap.as_str().to_string(), (cap.start() - 1) as i32);
                }
            }
        }

        path = self.params_regex.replace_all(&path, "*").to_string();

        for mut segment in path.split('/') {
            if segment.is_empty() {
                segment = "/";
            }

            let next_node = {
                let mut children = current_node.children.write().unwrap();

                children
                    .entry(segment.to_string())
                    .or_insert_with(|| Arc::new(TrieNode::default()))
                    .clone()
            };

            current_node = next_node;
        }

        current_node.params.write().unwrap().extend(path_params);

        *current_node.handler.write().unwrap() = Some(handler);
    }

    pub fn get_handler(&self, path: &str) -> (Option<RequestHandler>, HashMap<String, i32>) {
        let mut current_node = Arc::clone(&self.root);

        for mut segment in path.split('/') {
            if segment.is_empty() {
                segment = "/";
            }

            let next_node = {
                let children = current_node.children.read().unwrap();

                match children.get(segment) {
                    Some(route) => {
                        Some(Arc::clone(route))
                    }

                    None => {
                        match children.get("*") {
                            None => { None }
                            Some(route) => {
                                Some(Arc::clone(route))
                            }
                        }
                    }
                }
            };


            if let Some(r) = next_node {
                current_node = r; // Use the cloned Arc here
            } else {
                let params = current_node.params.read().unwrap().clone();

                return (None, params);
            }
        };

        let params = current_node.params.read().unwrap().clone();
        let handler = *current_node.handler.read().unwrap();

        (handler, params)
    }
}