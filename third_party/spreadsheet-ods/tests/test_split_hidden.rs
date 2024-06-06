use spreadsheet_ods::CellRange;

#[allow(dead_code)]
#[derive(Debug)]
struct SplitCols {
    col: u32,
    col_to: u32,
    hidden: bool,
}

/// Is the cell (partially) hidden?
fn split_hidden(ranges: &[CellRange], row: u32, col: u32, repeat: u32, out: &mut Vec<SplitCols>) {
    out.clear();
    if repeat == 1 {
        if ranges.iter().find(|v| v.contains(row, col)).is_some() {
            let range = SplitCols {
                col,
                col_to: col,
                hidden: true,
            };
            out.push(range);
        } else {
            let range = SplitCols {
                col,
                col_to: col,
                hidden: false,
            };
            out.push(range);
        }
    } else {
        let ranges: Vec<_> = ranges
            .iter()
            .filter(|v| row >= v.row() && row <= v.to_row())
            .collect();

        let mut range = SplitCols {
            col,
            col_to: col,
            hidden: false,
        };
        'col_loop: for c in col..col + repeat {
            for r in &ranges {
                if c >= r.col() && c <= r.to_col() {
                    if range.hidden {
                        range.col_to = c;
                    } else {
                        out.push(range);
                        range = SplitCols {
                            col: c,
                            col_to: c,
                            hidden: true,
                        }
                    }
                    continue 'col_loop;
                }
            }
            // not hidden
            if range.hidden {
                out.push(range);
                range = SplitCols {
                    col: c,
                    col_to: c,
                    hidden: false,
                }
            } else {
                range.col_to = c;
            }
        }
        out.push(range);
    }
}

#[test]
fn test_split_hidden() {
    let ranges = vec![CellRange::local(0, 1, 2, 3), CellRange::local(0, 6, 2, 8)];
    let mut s = Vec::new();

    split_hidden(&ranges, 1, 0, 1, &mut s);
    assert_eq!(s.len(), 1);
    split_hidden(&ranges, 1, 0, 4, &mut s);
    assert_eq!(s.len(), 2);
    split_hidden(&ranges, 1, 0, 6, &mut s);
    assert_eq!(s.len(), 3);
    split_hidden(&ranges, 1, 0, 7, &mut s);
    assert_eq!(s.len(), 4);
    split_hidden(&ranges, 1, 0, 10, &mut s);
    assert_eq!(s.len(), 5);

    split_hidden(&ranges, 1, 0, 1, &mut s);
    assert_eq!(s.len(), 1);
    split_hidden(&ranges, 1, 1, 1, &mut s);
    assert_eq!(s.len(), 1);
}
