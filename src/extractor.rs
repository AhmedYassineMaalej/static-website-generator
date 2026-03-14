use crate::markdown_node::MarkdownNode;
use crate::markdown_visitor::MarkdownVisitor;

pub trait Extractor<T>: MarkdownVisitor + Default {
    fn consume(self) -> T;
    fn extract(node: &impl MarkdownNode) -> T {
        let mut extractor = Self::default();
        node.accept(&mut extractor);
        extractor.consume()
    }
}
