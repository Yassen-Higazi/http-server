use crate::request::{HTTPMethod, Request};
use crate::response::Response;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub type RequestHandler = fn(&Request) -> Result<Response, Box<dyn Error>>;

#[derive(Default, Debug)]
struct TrieNode {
    params: RwLock<HashMap<String, i32>>,
    children: RwLock<HashMap<String, Arc<TrieNode>>>,
    handler: RwLock<HashMap<String, RequestHandler>>,
}

#[derive(Clone, Debug)]
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

    pub fn get(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::GET, path.to_string(), handler)
    }

    pub fn post(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::POST, path.to_string(), handler)
    }

    pub fn put(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::PUT, path.to_string(), handler)
    }

    pub fn patch(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::PATCH, path.to_string(), handler)
    }

    pub fn delete(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::DELETE, path.to_string(), handler)
    }

    pub fn options(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::OPTIONS, path.to_string(), handler)
    }

    pub fn head(&self, path: &str, handler: RequestHandler) -> &Router {
        self.define_route(&HTTPMethod::HEAD, path.to_string(), handler)
    }

    pub fn define_route(&self, method: &HTTPMethod, mut path: String, handler: RequestHandler) -> &Router {
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

        let mut handler_map = current_node.handler.write().unwrap();

        handler_map.insert(method.to_string(), handler);

        current_node.params.write().unwrap().extend(path_params);

        self
    }

    pub fn get_handler(&self, method: &HTTPMethod, path: &str) -> (Option<RequestHandler>, HashMap<String, i32>) {
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
        }
        
        let params = current_node.params.read().unwrap().clone();

        let handler_map = current_node.handler.read().unwrap();

        let handler = handler_map.get(&method.to_string()).cloned();

        (handler, params)
    }
}