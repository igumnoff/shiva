use spreadsheet_ods::xmltree::XmlTag;

#[test]
pub fn test_tree() {
    let _tag = XmlTag::new("table:shapes").tag(
        XmlTag::new("draw:frame")
            .attr_slice([
                ("draw:z", "0"),
                ("draw:name", "Bild 1"),
                ("draw:style:name", "gr1"),
                ("draw:text-style-name", "P1"),
                ("svg:width", "10.198cm"),
                ("svg:height", "1.75cm"),
                ("svg:x", "0cm"),
                ("svg:y", "0cm"),
            ])
            .tag(
                XmlTag::new("draw:image")
                    .attr_slice([
                        (
                            "xlink:href",
                            "Pictures/10000000000011D7000003105281DD09B0E0B8D4.jpg",
                        ),
                        ("xlink:type", "simple"),
                        ("xlink:show", "embed"),
                        ("xlink:actuate", "onLoad"),
                        ("loext:mime-type", "image/jpeg"),
                    ])
                    .tag(XmlTag::new("text:p")),
            ),
    );
}
