use std::fmt::Debug;
use bytes::Bytes;
use thiserror::Error;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {
    pub elements: Vec<Box<dyn Element>>,
    pub page_width: f32,
    pub page_height: f32,
    pub left_page_indent: f32,
    pub right_page_indent: f32,
    pub top_page_indent: f32,
    pub bottom_page_indent: f32,
}

impl Document {

    pub fn new(elements: &Vec<Box<dyn Element>>) -> anyhow::Result<Document> {
        Ok(Document {
            elements: (&elements).to_vec(),
            page_width: 210.0,
            page_height: 297.0,
            left_page_indent: 10.0,
            right_page_indent: 10.0,
            top_page_indent: 10.0,
            bottom_page_indent: 10.0,
        })
    }

}

pub trait TransformerTrait {
    fn parse(document: &Bytes, images: &HashMap<String, Bytes>) -> anyhow::Result<Document>;
    fn generate(document: &Document) -> anyhow::Result<(Bytes, HashMap<String, Bytes>)>;
}


#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parser error")]
    Common,
}
#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Generator error")]
    Common,
}

#[derive(Error, Debug)]
pub enum CastingError {
    #[error("CastingError error")]
    Common,
}

pub trait Element: CloneableElement + Debug {
    fn as_any(&self) -> &dyn Any;

    // fn yyy(a: &mut Box<A>) -> anyhow::Result<&mut Box<A>> {

    fn as_any_mut(&mut self) -> &mut dyn Any;


    fn element_type(&self) -> ElementType;



    fn paragraph_as_ref(&self) -> anyhow::Result<&ParagraphElement> {
        Ok(self.as_any().downcast_ref::<ParagraphElement>().ok_or(CastingError::Common)?)
    }

    fn paragraph_as_mut1(&mut self) -> anyhow::Result<&mut ParagraphElement> {
        Ok(self.as_any_mut().downcast_mut::<ParagraphElement>().ok_or(CastingError::Common)?)
    }

}

pub trait CloneableElement {
    fn clone_box(&self) -> Box<dyn Element>;
}

impl<T> CloneableElement for T
    where
        T: 'static + Element + Clone,
{
    fn clone_box(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Element> {
    fn clone(&self) -> Box<dyn Element> {
        self.clone_box()
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Text,
    Paragraph,
    Image,
    Hyperlink,
    Header,
    Table,
    TableHeader,
    TableRow,
    TableCell,
    List,
    ListItem,
    PageBreak,
    TableOfContents,
}


#[derive(Clone, Debug)]
pub struct TextElement {
    pub text: String,
    pub size: u8,
}

impl TextElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&TextElement> {
        Ok(element.as_any().downcast_ref::<TextElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(text: &str, size: u8) -> anyhow::Result<Box<dyn Element>> {
        Ok(Box::new(TextElement {
            text: text.to_string(),
            size,
        }))
    }
}

impl Element for TextElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::Text
    }


}


#[derive(Clone, Debug)]
pub struct HeaderElement {
    pub level: u8,
    pub text: String,
    pub children: Vec<Box<dyn Element>>,
}

impl HeaderElement {
    pub fn as_ref(element: &Box<dyn Element>) -> anyhow::Result<&HeaderElement> {
        Ok(element.as_any().downcast_ref::<HeaderElement>().ok_or(CastingError::Common)?)
    }
    pub fn as_mut(element: &mut Box<dyn Element>) -> anyhow::Result<&mut HeaderElement> {
        Ok(element.as_any_mut().downcast_mut::<HeaderElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(text: &str, level: u8) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(HeaderElement {
            level,
            text: text.to_string(),
            children: vec![],
        }))
    }
}

impl Element for HeaderElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }


    fn element_type(&self) -> ElementType {
        ElementType::Header
    }


}


#[derive(Clone, Debug)]
pub struct ParagraphElement {
    pub elements: Vec<Box<dyn Element>>,
}

impl ParagraphElement {
    pub fn as_ref(element: &Box<dyn Element>) -> anyhow::Result<&ParagraphElement> {
        Ok(element.as_any().downcast_ref::<ParagraphElement>().ok_or(CastingError::Common)?)
    }

    pub fn as_mut(element: &mut Box<dyn Element>) -> anyhow::Result<&mut ParagraphElement> {
        Ok(element.as_any_mut().downcast_mut::<ParagraphElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(elements: &Vec<Box<dyn Element>>) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(ParagraphElement {
            elements: (&elements).to_vec(),
        }))
    }

}

impl Element for ParagraphElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::Paragraph
    }

}


#[derive(Clone, Debug)]
pub struct TableElement {
    pub rows: Vec<TableRowElement>,
    pub headers: Vec<TableHeaderElement>,
}

impl TableElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&TableElement> {
        Ok(element.as_any().downcast_ref::<TableElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(headers: &Vec<TableHeaderElement>, rows: &Vec<TableRowElement>) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(TableElement {
            rows: rows.to_vec(),
            headers: headers.to_vec(),
        }))
    }

}

impl Element for TableElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::Table
    }
}

#[derive(Clone, Debug)]
pub struct TableHeaderElement {
    pub element: Box<dyn Element>,
}

impl TableHeaderElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&TableHeaderElement> {
        Ok(element.as_any().downcast_ref::<TableHeaderElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(element: &Box<dyn Element>) -> anyhow::Result<TableHeaderElement> {
        Ok(TableHeaderElement {
            element: element.clone(),
        })
    }


}


impl Element for TableHeaderElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::TableHeader
    }
}

#[derive(Clone, Debug)]
pub struct TableRowElement {
    pub cells: Vec<TableCellElement>,
}

impl TableRowElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&TableRowElement> {
        Ok(element.as_any().downcast_ref::<TableRowElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(cells: &Vec<TableCellElement>) -> anyhow::Result<TableRowElement> {
        Ok(TableRowElement {
            cells: cells.to_vec(),
        })
    }

}

impl Element for TableRowElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::TableRow
    }
}

#[derive(Clone, Debug)]
pub struct TableCellElement {
    pub element: Box<dyn Element>,
}

impl TableCellElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&TableCellElement> {
        Ok(element.as_any().downcast_ref::<TableCellElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(element: &Box<dyn Element>) -> anyhow::Result<TableCellElement> {
        Ok(TableCellElement {
            element: element.clone(),
        })
    }

}

impl Element for TableCellElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::TableCell
    }
}

#[derive(Clone, Debug)]
pub struct ListElement {
    pub elements: Vec<ListItemElement>,
    pub numbered: bool,
}

impl ListElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&ListElement> {
        Ok(element.as_any().downcast_ref::<ListElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(elements: &Vec<ListItemElement>, numbered: bool) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(ListElement {
            elements: elements.to_vec(),
            numbered,
        }))
    }
}

impl Element for ListElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::List
    }
}

#[derive(Clone, Debug)]
pub struct ListItemElement {
    pub element: Box<dyn Element>,
}

impl ListItemElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&ListItemElement> {
        Ok(element.as_any().downcast_ref::<ListItemElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(element: &Box<dyn Element>) -> anyhow::Result<ListItemElement> {
        Ok(ListItemElement {
            element: element.clone(),
        })
    }

}

impl Element for ListItemElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::ListItem
    }
}

#[derive(Clone, Debug)]
pub enum ImageType {
    Png,
    Jpeg,
}

#[derive(Clone, Debug)]
pub struct ImageElement {
    pub bytes: Bytes,
    pub title: String,
    pub alt: String,
    pub image_type: ImageType,
}

impl ImageElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&ImageElement> {
        Ok(element.as_any().downcast_ref::<ImageElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(bytes: &Bytes, title: &str, alt: &str, image_type: ImageType) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(ImageElement {
            bytes: bytes.clone(),
            title: title.to_string(),
            alt: alt.to_string(),
            image_type,
        }))
    }
}

impl Element for ImageElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::Image
    }
}

#[derive(Clone, Debug)]
pub struct HyperlinkElement {
    pub title: String,
    pub url: String,
    pub alt: String,
}

impl HyperlinkElement {
    pub fn from(element: &Box<dyn Element>) -> anyhow::Result<&HyperlinkElement> {
        Ok(element.as_any().downcast_ref::<HyperlinkElement>().ok_or(CastingError::Common)?)
    }

    pub fn new(title: &str, url: &str, alt: &str) -> anyhow::Result<Box<dyn Element>> where Self: Sized {
        Ok(Box::new(HyperlinkElement {
            title: title.to_string(),
            url: url.to_string(),
            alt: alt.to_string(),
        }))
    }
}

impl Element for HyperlinkElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn element_type(&self) -> ElementType {
        ElementType::Hyperlink
    }
}