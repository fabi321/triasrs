use std::sync::Arc;
use sxd_xpath::nodeset::Node;
use crate::data_types::FromNode;
use crate::data_types::transport_modes::TransportMode;
use crate::util::Point;

/// A stop as in bus/train/tram stop
pub struct Stop {
    id: Arc<String>,
    name: String,
    locality: String,
    // mostly city
    location: String,
    position: Point,
    modes: Vec<TransportMode>,
}

impl Stop {
    pub fn new(
        id: Arc<String>,
        name: String,
        locality: String,
        location: String,
        position: Point,
        modes: Vec<TransportMode>
    ) -> Stop {
        Stop {id, name, locality, location, position, modes}
    }
}

impl FromNode for Stop {
    fn from_node(from: Node) -> Option<Self> {
        #[cfg(debug)]
        assert_eq!(from.attribute().map(|a| a.name().local_part()), Some("LocationResult"));
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut locality: Option<String> = None;
        let mut location: Option<String> = None;
        let mut position: Option<Point> = None;
        let mut modes: Vec<TransportMode> = vec![];
        for node in from.children() {
            if let Some(name) = node.attribute().map(|a| a.name().local_part()) {
                if name == "Location" {
                    for node in node.children() {
                        if let Some(name) = node.attribute().map(|a| a.name().local_part()) {
                            if name == "StopPoint" {

                            }
                        }
                    }
                }
            }
        }
        unimplemented!()
    }
}
