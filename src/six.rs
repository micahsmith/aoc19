use std::collections::HashMap;
use std::sync::Arc;

pub fn start(input: &str) {
    let mut map: HashMap<&str, Arc<OrbitNode>> = HashMap::new();

    input
        .trim()
        .split("\n")
        .map(|orbit| parse_orbit(orbit))
        .for_each(|(orbitted, orbitter)| {
            let mut node = OrbitNode::new(orbitter);
            if let Some(p) = map.get(orbitted) {
                node.clone().add_parent(p);
            }

            map.insert(orbitter, node);
            



        });
}

#[derive(Clone, Debug)]
struct OrbitNode {
    name: String,
    children: Vec<Arc<OrbitNode>>,
    parent: Option<Arc<OrbitNode>>,
}

impl OrbitNode {
    pub fn new(name: &str) -> Arc<Self> {
        return Arc::new(OrbitNode {
            name: String::from(name),
            children: Vec::new(),
            parent: None,
        });
    }

    pub fn add_child(&mut self, child: &Arc<Self>) {
        self.children.push(child.clone());
    }

    pub fn add_parent(&mut self, parent: &Arc<Self>) {
        self.parent = Some(parent.clone());
    }
}

fn parse_orbit(orbit: &str) -> (&str, &str) {
    let mut parsed = orbit.split(")");
    return (parsed.next().unwrap(), parsed.next().unwrap());
}
