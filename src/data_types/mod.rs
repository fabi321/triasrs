use sxd_xpath::nodeset::Node;

mod stop;
pub mod transport_modes;

pub trait FromNode: Sized {
    fn from_node(from: Node) -> Option<Self>;
}