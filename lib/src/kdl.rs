use crate::core::*;
use bytes::Bytes;
use std::collections::HashMap;
use kdl::{KdlDocument, KdlNode};

use crate::core::Element::{Header, Hyperlink, Image, List, Paragraph, Table, Text};

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document> {

    }
    
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {

    }
}