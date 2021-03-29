use super::{Link, Links};
use crate::http::SimpleRespondable;
use actix_http::http::header::ContentType;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Representation of a HAL document.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct HalDocument {
    /// The actual data in the document.
    #[serde(flatten)]
    pub data: Value,

    /// The set of linsk in the document.
    #[serde(rename = "_links", skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, Links>,
}

impl HalDocument {
    /// Create a new document for the provided data
    ///
    /// - `data` - The data to represent in the HAL document.
    pub fn new<T>(data: T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_value(data).unwrap();

        Self {
            data,
            links: BTreeMap::new(),
        }
    }

    /// Add a new link to the document
    ///
    /// - `rel` - The link relation  of the link
    /// - `link` - The actual link
    pub fn with_link<N, L>(mut self, rel: N, link: L) -> Self
    where
        N: Into<String>,
        L: Into<Link>,
    {
        let rel = rel.into();
        let link = link.into();

        let links = match self.links.remove(&rel) {
            None => Links::Single(link),
            Some(links) => links.append(link),
        };

        self.links.insert(rel, links);

        self
    }
}

impl From<HalDocument> for SimpleRespondable<HalDocument> {
    fn from(body: HalDocument) -> Self {
        let content_type = ContentType("application/hal+json".parse().unwrap());

        SimpleRespondable::new(body).with_header(content_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Respondable;
    use assert2::{check, let_assert};
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct Body {
        pub name: String,
    }

    #[test]
    fn new_from_struct() {
        let body = Body {
            name: "Graham".to_owned(),
        };
        let document = HalDocument::new(body);

        check!(document.data == json!({"name": "Graham"}));
        check!(document.links.is_empty());
    }

    #[test]
    fn with_single_links() {
        let body = Body {
            name: "Graham".to_owned(),
        };
        let document = HalDocument::new(body)
            .with_link("self", "/")
            .with_link("author", "/users/abc");

        check!(document.links.len() == 2);

        let_assert!(Some(Links::Single(self_link)) = document.links.get("self"));
        check!(self_link == &Link::from("/"));

        let_assert!(Some(Links::Single(author_link)) = document.links.get("author"));
        check!(author_link == &Link::from("/users/abc"));
    }

    #[test]
    fn with_repeated_links() {
        let body = Body {
            name: "Graham".to_owned(),
        };
        let document = HalDocument::new(body)
            .with_link("item", "/foo")
            .with_link("item", "/bar");

        check!(document.links.len() == 1);

        let_assert!(Some(Links::Multiple(links)) = document.links.get("item"));
        check!(links == &vec![Link::from("/foo"), Link::from("/bar")]);
    }

    #[test]
    fn to_respondable() {
        let document = HalDocument::new(Body {
            name: "Graham".to_owned(),
        });
        let respondable = SimpleRespondable::from(document);

        check!(respondable.status_code() == 200);

        let headers = respondable.headers();
        let content_type = headers.get("content-type");
        check!(content_type.unwrap() == "application/hal+json");

        let body = respondable.body();
        check!(
            body == HalDocument::new(Body {
                name: "Graham".to_owned(),
            })
        );
    }
}
