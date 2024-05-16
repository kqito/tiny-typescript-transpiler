#[macro_export]
macro_rules! impl_node_from {
    ($generic:ty, $( $from:ty => $to:expr ),+) => ($(
        impl From<$from> for $generic {
            fn from(val: $from) -> Self {
                $to(val)
            }
        }

        impl From<Node<$from>> for Node<$generic> {
            fn from(node: Node<$from>) -> Node<$generic> {
                Node { loc: node.loc, item: $to(node.item) }
            }
        }
    )*)
}
