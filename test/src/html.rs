#[cfg(test)]
mod tests {
    use shiva::core::*;
    use shiva::html::Transformer;
    use shiva::markdown;
    use std::collections::HashMap;

    #[test]
    fn test_html_header_parse() -> anyhow::Result<()> {
        let html_document = r#"
      <!DOCTYPE html>
      <html>
      <body>
      <h1>First header</h1>
      <h2>Second header</h2>
      <h3>Third header</h3>
      <h4>Fourth header</h4>
      <h5>Fifth header</h5>
      <h6>Sixth header</h6>
     </body>
      </html>
              "#;

        println!("==========================");
        let parsed = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());
        println!("{:?}", &parsed);
        println!("==========================");

        let generated_result = Transformer::generate(&parsed?);
        assert!(generated_result.is_ok());
        // println!("{:?}", generated_result.unwrap());
        let generated_bytes = generated_result?;
        let generated_text = std::str::from_utf8(&generated_bytes.0)?;
        println!("{}", generated_text);

        Ok(())
    }
    /*
        #[test]
        fn test() -> anyhow::Result<()> {
            let document = r#"# First header

    Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
    blabla bla bla blabla bla bla blabla bla bla blabla bla bla bla

    1. List item 1
    2. List item 2
    3. List item 3
       1. List item secode level 1
       2. List item secode level 2
    4. List item 4
       1. List item secode level 3
       2. List item secode level 4
    5. List item 5
       1. List item secode level 5

    - List item one
      - List item two
    - List item three
      - List item four
      - List item five
        - List item zzz
    - List item six
      - List item seven

    ![Picture alt1](test/data/picture.png "Picture title1")


    Bla bla bla ![Picture alt2](test/data/picture.png "Picture title2") bla. http://example.com  [Example](http://example.com) [Example](http://example.com "Example tooltip")


    ## Second header

    | Syntax      | Description |
    | ----------- | ----------- |
    | Header      | Title       |
    | Paragraph   | Text        |

    Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla
    blabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla"#;
            // println!("{:?}", document);
            let mut images = HashMap::new();
            let image_bytes = std::fs::read("data/picture.png")?;
            images.insert("data/picture.png".to_string(), image_bytes);
            let parsed = markdown::Transformer::parse(&document.as_bytes().into(), &HashMap::new());
            let document_string = std::str::from_utf8(document.as_bytes())?;
            println!("{}", document_string);
            assert!(parsed.is_ok());
            let parsed_document = parsed.unwrap();
            println!("==========================");
            println!("{:?}", parsed_document);
            println!("==========================");
            let generated_result = markdown::Transformer::generate(&parsed_document);
            assert!(generated_result.is_ok());
            // println!("{:?}", generated_result.unwrap());
            let generated_bytes = generated_result?;
            let generated_text = std::str::from_utf8(&generated_bytes.0)?;
            println!("{}", generated_text);

            let html_document = r#"
    <!DOCTYPE html>
    <html>
    <body>
    <h1>First header</h1>
    <p>Paragraph  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla blablabla bla bla blabla bla bla blabla bla bla blabla bla bla bla</p>
    <ol>
    <li>List item 1</li>
    <li>List item 2</li>
    <li>List item 3</li>
    <li><ol>
    <li>List item secode level 1</li>
    <li>List item secode level 2</li>
    </ol>
    </li>
    <li>List item 4</li>
    <li><ol>
    <li>List item secode level 3</li>
    <li>List item secode level 4</li>
    </ol>
    </li>
    <li>List item 5</li>
    <li><ol>
    <li>List item secode level 5</li>
    </ol>
    </li>
    </ol>
    <ul>
    <li>List item one</li>
    <li><ul>
    <li>List item two</li>
    </ul>
    </li>
    <li>List item three</li>
    <li><ul>
    <li>List item four</li>
    <li>List item five</li>
    <li><ul>
    <li>List item zzz</li>
    </ul>
    </li>
    </ul>
    </li>
    <li>List item six</li>
    <li><ul>
    <li>List item seven</li>
    </ul>
    </li>
    </ul>
    <p><img src="test/data/image0.png" alt="Picture alt1" title="Picture title1" /></p>
    <p>Bla bla bla <img src="test/data/image1.png" alt="Picture alt2" title="Picture title2" /> bla. <a href="http://example.com" title="http://example.com">http://example.com</a>  <a href="http://example.com" title="Example">Example</a> <a href="http://example.com" title="Example tooltip">Example</a></p>
    <h2>Second header</h2>
    <table  border="1">
    <tr>
    <th>Syntax</th>
    <th>Description</th>
    </tr>
    <tr>
    <td>Header</td>
    <td>Title</td>
    </tr>
    <tr>
    <td>Paragraph</td>
    <td>Text</td>
    </tr>
    </table>
    <p>Paragraph2  bla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla bla blabla bla blablabla2 bla bla blabla bla bla blabla bla bla blabla bla bla bla</p>
    </body>
    </html>
            "#;

            println!("==========================");
            let mut images = HashMap::new();
            let image_bytes = std::fs::read("data/image0.png")?;
            images.insert("image0.png".to_string(), image_bytes);
            let image_bytes = std::fs::read("data/image1.png")?;
            images.insert("image1.png".to_string(), image_bytes);
            let parsed = Transformer::parse(&html_document.as_bytes().into(), &HashMap::new());
            println!("{:?}", &parsed);
            println!("==========================");

            let generated_result = Transformer::generate(&parsed?);
            assert!(generated_result.is_ok());
            // println!("{:?}", generated_result.unwrap());
            let generated_bytes = generated_result?;
            let generated_text = std::str::from_utf8(&generated_bytes.0)?;
            println!("{}", generated_text);

            Ok(())
        }
        */
}
