use std::collections::HashMap;
use bytes::Bytes;
use lopdf::{Document};
use crate::core::TransformerTrait;

pub struct Transformer;
impl TransformerTrait for Transformer {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<crate::core::Document> {
        let mut doc = Document::with_version("1.5");



        todo!()
    }

    fn generate(document: &crate::core::Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)> {
        todo!()
    }
}
