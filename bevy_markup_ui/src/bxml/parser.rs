use anyhow::Result;
use bevy::{prelude::error, utils::HashMap};
use roxmltree::{Document, Node};

#[derive(Debug, PartialEq, Clone)]
pub struct Bxml {
    pub templates: HashMap<String, Template>,
    pub children: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Template {
    pub src: String,
    pub tag: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    StyleSheet(StyleSheet),
    Bundle(Bundle),
    Component(Component),
    Instance(Instance),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyleSheet {
    pub src: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bundle {
    pub children: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Component {
    pub ty: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub tag: String,
    pub children: Vec<Element>,
}

impl Bxml {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_str(std::str::from_utf8(bytes)?)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let doc = Document::parse(s)?;
        Ok(Self::from_node(&doc.root_element()))
    }

    pub fn from_node(node: &Node) -> Self {
        let templates = gather_templates(&node);
        let children = gather_elements(&node);
        Self {
            templates,
            children,
        }
    }
}

impl Template {
    pub fn from_node(node: &Node) -> Option<Self> {
        let Some(src) = node.attribute("src").map(|s| s.to_string()) else {
            error!("found Template without src attribute");
            return None;
        };
        let Some(tag) = node.attribute("tag").map(|s| s.to_string()) else {
            error!("found Template without tag attribute");
            return None;
        };
        Some(Self { src, tag })
    }
}

impl Element {
    pub fn from_node(node: &Node) -> Option<Self> {
        if node.is_text() {
            let text = node.text()?.trim();
            if text.is_empty() {
                return None;
            }
            return Some(Self::Text(text.to_string()));
        }
        Some(match node.tag_name().name() {
            "StyleSheet" => Self::StyleSheet(StyleSheet::from_node(&node)?),
            "Bundle" => Self::Bundle(Bundle::from_node(&node)?),
            "Component" => Self::Component(Component::from_node(&node)?),
            "Template" => return None,
            _ => Self::Instance(Instance::from_node(&node)?),
        })
    }
}

impl StyleSheet {
    pub fn from_node(node: &Node) -> Option<Self> {
        let Some(src) = node.attribute("src").map(|s| s.to_string()) else {
            error!("found StyleSheet without src attribute");
            return None;
        };
        Some(Self { src })
    }
}

impl Bundle {
    pub fn from_node(node: &Node) -> Option<Self> {
        let children = gather_elements(node);
        if children.is_empty() {
            error!("found Bundle without children");
            return None;
        }
        Some(Self { children })
    }
}

impl Component {
    pub fn from_node(node: &Node) -> Option<Self> {
        let Some(ty) = node.attribute("type").map(|s| s.to_string()) else {
            error!("found Component without ty attribute");
            return None;
        };
        Some(Self { ty })
    }
}

impl Instance {
    pub fn from_node(node: &Node) -> Option<Self> {
        let tag = node.tag_name().name().to_string();
        if tag.is_empty() {
            error!("found Instance without tag");
            return None;
        }
        let children = gather_elements(node);
        Some(Self { tag, children })
    }
}

fn gather_templates(node: &Node) -> HashMap<String, Template> {
    node.descendants()
        .filter_map(|node| match node.tag_name().name() {
            "Template" => {
                let template = Template::from_node(&node)?;
                Some((template.tag.clone(), template))
            }
            _ => None,
        })
        .collect()
}

fn gather_elements(node: &Node) -> Vec<Element> {
    node.children()
        .filter_map(|node| Element::from_node(&node))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bxml() {
        let text = r#"<BXML>
    <Template src="some/path" tag="Foo" />
    <Foo />
    <Bundle>
        <Template src="some/path" tag="Bar" />
        <Component type="Node" />
        <StyleSheet src="some/path" />
    </Bundle>
</BXML>"#;
        let templates = vec![
            Template {
                src: "some/path".to_string(),
                tag: "Foo".to_string(),
            },
            Template {
                src: "some/path".to_string(),
                tag: "Bar".to_string(),
            },
        ];
        let children = vec![
            Element::Instance(Instance {
                tag: "Foo".to_string(),
                children: Vec::new(),
            }),
            Element::Bundle(Bundle {
                children: vec![
                    Element::Component(Component {
                        ty: "Node".to_string(),
                    }),
                    Element::StyleSheet(StyleSheet {
                        src: "some/path".to_string(),
                    }),
                ],
            }),
        ];
        assert_eq!(
            Bxml::from_str(text).unwrap(),
            Bxml {
                templates: templates
                    .iter()
                    .cloned()
                    .map(|template| (template.tag.clone(), template))
                    .collect(),
                children,
            }
        )
    }

    #[test]
    fn test_template_from_node() {
        let doc = doc_parse(r#"<Template src="some/path" tag="Foo" />"#);
        assert_eq!(
            Template::from_node(&doc.root_element()),
            Some(Template {
                src: "some/path".to_string(),
                tag: "Foo".to_string(),
            }),
        );
    }

    #[test]
    fn test_style_sheet_from_node() {
        let doc = doc_parse(r#"<StyleSheet src="some/path" />"#);
        assert_eq!(
            StyleSheet::from_node(&doc.root_element()),
            Some(StyleSheet {
                src: "some/path".to_string()
            }),
        );
    }

    #[test]
    fn test_bundle_from_node() {
        let doc = doc_parse(r#"<Bundle><Component type="Node" /><Button /></Bundle>"#);
        assert_eq!(
            Bundle::from_node(&doc.root_element()),
            Some(Bundle {
                children: vec![
                    Element::Component(Component {
                        ty: "Node".to_string(),
                    }),
                    Element::Instance(Instance {
                        tag: "Button".to_string(),
                        children: Vec::new()
                    })
                ],
            }),
        );
    }

    #[test]
    fn test_component_from_node() {
        let doc = doc_parse(r#"<Component type="Node" />"#);
        assert_eq!(
            Component::from_node(&doc.root_element()),
            Some(Component {
                ty: "Node".to_string(),
            }),
        );
    }

    #[test]
    fn test_instance_from_node() {
        let doc = doc_parse(r#"<Button><Component type="Node" /></Button>"#);
        assert_eq!(
            Instance::from_node(&doc.root_element()),
            Some(Instance {
                tag: "Button".to_string(),
                children: vec![Element::Component(Component {
                    ty: "Node".to_string()
                })],
            }),
        );
    }

    #[test]
    fn test_text_from_node() {
        let doc = doc_parse(r#"<BXML>Just some text.</BXML>"#);
        assert_eq!(
            Element::from_node(&doc.root_element().first_child().unwrap()),
            Some(Element::Text("Just some text.".to_string())),
        );

        let doc = doc_parse(
            r#"<BXML>
             
                </BXML>"#,
        );
        assert_eq!(
            Element::from_node(&doc.root_element().first_child().unwrap()),
            None,
        );
    }

    fn doc_parse(text: &str) -> Document {
        Document::parse(text).unwrap()
    }
}
