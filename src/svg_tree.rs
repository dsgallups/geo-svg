use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use crate::ViewBox;

pub struct SvgTree {
    // main content
    pub tag: String,
    pub content: SvgTreeChildren,
    // BTreeMap mainly for fixed order and uniqueness of keys
    pub attrs: BTreeMap<String, String>,

    // meta
    pub id: Option<String>,
    pub viewbox: Option<ViewBox>,
}

pub enum SvgTreeChildren {
    Content(String),
    Children(Vec<SvgTree>),
}

impl SvgTree {
    pub fn root() -> Self {
        Self {
            tag: String::from("svg"),
            content: SvgTreeChildren::Children(vec![]),
            attrs: BTreeMap::from_iter(
                [
                    ("xmlns", "http://www.w3.org/2000/svg"),
                    ("preserveAspectRatio", "xMidYMid meet"),
                ]
                .map(|(a, b)| (a.to_string(), b.to_string())),
            ),
            id: None,
            viewbox: Some(ViewBox::default()),
        }
    }

    pub fn leaf(tag: impl AsRef<str>, content: impl AsRef<str>) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            content: SvgTreeChildren::Content(content.as_ref().to_string()),
            attrs: BTreeMap::new(),
            id: None,
            viewbox: None,
        }
    }

    pub fn add(mut self, child: Self) -> Self {
        if let SvgTreeChildren::Children(children) = &mut self.content {
            children.push(child);
        }
        self
    }
}

impl Display for SvgTreeChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgTreeChildren::Content(content) => write!(f, "{content}"),
            SvgTreeChildren::Children(children) => write!(
                f,
                "{children}",
                children = children
                    .iter()
                    .map(|child| child.to_string())
                    .reduce(|a, b| a + b.as_str())
                    .unwrap_or_default()
            ),
        }
    }
}

impl Debug for SvgTreeChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgTreeChildren::Content(content) => write!(f, "\n{content}\n"),
            SvgTreeChildren::Children(children) => write!(
                f,
                "{children}",
                children = children
                    .iter()
                    .map(|child| format!("{child:?}"))
                    .reduce(|a, b| a + "\n" + b.as_str())
                    .map(|content| format!("\n{content}\n"))
                    .unwrap_or_default()
            ),
        }
    }
}

impl Display for SvgTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vb = self
            .viewbox
            .as_ref()
            .map(|vb| (String::from("viewBox"), vb.to_string()));
        write!(
            f,
            "<{tag}{attrs}>{content}</{tag}>",
            tag = self.tag,
            attrs = self
                .attrs
                .clone()
                .into_iter()
                .chain(vb.into_iter())
                .map(|(key, value)| format!("{key}=\"{value}\""))
                .reduce(|a, b| a + " " + b.as_str())
                .map(|attrs| format!(" {attrs}"))
                .unwrap_or_default(),
            content = self.content.to_string()
        )
    }
}

impl Debug for SvgTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vb = self
            .viewbox
            .as_ref()
            .map(|vb| (String::from("viewBox"), vb.to_string()));
        write!(
            f,
            "<{tag}{attrs}>{content}</{tag}>",
            tag = self.tag,
            attrs = self
                .attrs
                .clone()
                .into_iter()
                .chain(vb.into_iter())
                .map(|(key, value)| format!("{key}=\"{value}\""))
                .reduce(|a, b| a + " " + b.as_str())
                .map(|attrs| format!(" {attrs}"))
                .unwrap_or_default(),
            content = format!("{:?}", self.content)
                .lines()
                .map(|line| (!line.trim().is_empty())
                    .then(|| format!("  {line}"))
                    .unwrap_or_default())
                .reduce(|a, b| a + "\n" + b.as_str())
                .map(|content| format!("{content}\n"))
                .unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod svg_tree_tests {
    use super::*;

    #[test]
    fn empty_root() {
        let root = SvgTree::root();
        assert_eq!(
            format!("{root}"),
            r#"
<svg preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 0 0"></svg>
"#
            .trim()
        );
        assert_eq!(
            format!("{root:?}"),
            r#"
<svg preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 0 0"></svg>
"#
            .trim()
        );
    }

    #[test]
    fn just_leaf() {
        let leaf = SvgTree::leaf("abc", "def");
        assert_eq!(
            format!("{leaf}"),
            r#"
<abc>def</abc>
"#
            .trim()
        );
        assert_eq!(
            format!("{leaf:?}"),
            r#"
<abc>
  def
</abc>
"#
            .trim()
        );
    }

    #[test]
    fn small_tree() {
        let svg = SvgTree::root()
            .add(SvgTree::leaf("abc", "def"))
            .add(SvgTree::leaf("hij", "lmnop"));

        assert_eq!(
            format!("{svg}"),
            r#"
<svg preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 0 0"><abc>def</abc><hij>lmnop</hij></svg>
"#
            .trim()
        );
        assert_eq!(
            format!("{svg:?}"),
            r#"
<svg preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 0 0">
  <abc>
    def
  </abc>
  <hij>
    lmnop
  </hij>
</svg>
"#
            .trim()
        );
    }
}
