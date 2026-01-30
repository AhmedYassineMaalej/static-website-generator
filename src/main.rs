mod htmlnode;
mod leafnode;
mod properties;
mod textnode;

use textnode::TextNode;

fn main() {
    let node = TextNode::Link {
        text: String::from("This is some text"),
        url: String::from("https://personal-blog-two-gray.vercel.app"),
    };
}
