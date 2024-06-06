// I'm forced to put the functions in the test because they are private outside the crate
fn push_rowname(buf: &mut String, row: u32) {
    let row: u64 = row as u64 + 1;
    buf.push_str(&row.to_string());
}

fn push_colname(buf: &mut String, col: u32) {
    let mut col: u64 = col as u64 + 1;
    let mut _buf = String::new();

    while col != 0 {
        _buf.push(std::char::from_u32((col % 26) as u32 - 1 + 'A' as u32).unwrap());
        col /= 26;
    }

    buf.push_str(&_buf.chars().rev().collect::<String>());
}

// the old function
/// Appends the spreadsheet column name.
fn old_push_colname(buf: &mut String, mut col: u32) {
    let mut i = 0;
    let mut dbuf = [0u8; 7];

    col += 1;
    while col > 0 {
        dbuf[i] = (col % 26) as u8;
        if dbuf[i] == 0 {
            dbuf[i] = 25;
            col = col / 26 - 1;
        } else {
            dbuf[i] -= 1;
            col /= 26;
        }

        i += 1;
    }

    // reverse order
    let mut j = i;
    while j > 0 {
        buf.push((b'A' + dbuf[j - 1]) as char);
        j -= 1;
    }
}

// the old function
/// Appends the spreadsheet row name
fn old_push_rowname(buf: &mut String, mut row: u32) {
    let mut i = 0;
    let mut dbuf = [0u8; 10];

    row += 1;
    while row > 0 {
        dbuf[i] = (row % 10) as u8;
        row /= 10;

        i += 1;
    }

    // reverse order
    let mut j = i;
    while j > 0 {
        buf.push((b'0' + dbuf[j - 1]) as char);
        j -= 1;
    }
}

#[test]
fn issue6() {
    let mut old_buf = String::new();
    let mut buf = String::new();

    push_rowname(&mut buf, 0);
    old_push_rowname(&mut old_buf, 0);
    push_rowname(&mut buf, 2);
    old_push_rowname(&mut old_buf, 2);
    push_rowname(&mut buf, 24);
    old_push_rowname(&mut old_buf, 24);
    push_rowname(&mut buf, 3523462353);
    old_push_rowname(&mut old_buf, 3523462353);
    assert_eq!(old_buf, buf);

    buf.clear();
    old_buf.clear();

    push_colname(&mut buf, 0);
    old_push_colname(&mut old_buf, 0);
    push_colname(&mut buf, 2);
    old_push_colname(&mut old_buf, 2);
    push_colname(&mut buf, 24);
    old_push_colname(&mut old_buf, 24);
    push_colname(&mut buf, 3523462353);
    old_push_colname(&mut old_buf, 3523462353);
    assert_eq!(old_buf, buf);

    // overlfow test
    push_colname(&mut buf, u32::MAX);
    push_rowname(&mut buf, u32::MAX);
}
