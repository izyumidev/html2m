use crate::structs::{Node, NodeType::*};

pub fn to_md(node: Node) -> String {
    let mut res = String::new();
    let mut tail = String::new();

    let mut follow_child = true; // If the function should process the children of the node, defaults to true. False for some tags; like <ul> and <ol>.

    if let Some(tag_type) = node.tag_name {
        match tag_type {
            H1 | H2 | H3 | H4 | H5 | H6 => tail.push('\n'),
            _ => (),
        }
        match tag_type {
            H1 => res.push_str("# "),
            H2 => res.push_str("## "),
            H3 => res.push_str("### "),
            H4 => res.push_str("#### "),
            H5 => res.push_str("##### "),
            H6 => res.push_str("###### "),
            Strong => {
                res.push_str("**");
                tail.push_str("**");
            }
            Em => {
                res.push('*');
                tail.push('*');
            }
            A => {
                if let Some(link) = node.attributes.as_ref().and_then(|attrs| attrs.get("href")) {
                    res.push('[');
                    tail.push_str(&format!("]({})", link));
                } else {
                    res.push('[');
                    tail.push(']');
                }
            }
            Ul => {
                for child in &node.children {
                    res.push_str("- ");
                    res.push_str(&to_md(child.clone()));
                }
                follow_child = false;
            }
            Ol => {
                let mut i = 1;
                for child in &node.children {
                    res.push_str(&format!("{}. ", i));
                    res.push_str(&to_md(child.clone()));
                    i += 1;
                }
                follow_child = false;
            }
            Li => {
                tail.push('\n');
            }
            P => {
                if node.children.is_empty() {
                    return res;
                }
                tail.push('\n');
            }
            Code => {
                if let Some(language) = node
                    .attributes
                    .as_ref()
                    .and_then(|attrs| attrs.get("class"))
                    .unwrap_or(&"".to_string())
                    .split_whitespace()
                    .find(|class| class.starts_with("language-"))
                    .map(|class| &class[9..])
                {
                    res.push_str(&format!("```{}\n", language));
                } else {
                    res.push_str("```\n");
                }
                tail.push_str("```\n");
            }
            Hr => {
                res.push_str("***\n");
                follow_child = false;
            }
            Text => {
                res.push_str(&node.value.unwrap_or("".to_string()));
                return res;
            }
            Div | Pre => (),
        }
    }

    if follow_child {
        for child in node.children {
            res.push_str(&to_md(child));
        }
    }

    res.push_str(&tail);

    res
}

pub fn from_html_to_md(input: String) -> String {
    let node = crate::parser::parse_html(input);
    to_md(node)
}
