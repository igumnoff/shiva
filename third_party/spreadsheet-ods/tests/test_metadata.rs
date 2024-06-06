mod lib_test;

use chrono::{Duration, NaiveDateTime};
use lib_test::*;
use spreadsheet_ods::metadata::{MetaUserDefined, MetaValue};
use spreadsheet_ods::{read_ods, OdsError, Sheet, WorkBook};

#[test]
fn test_write_read() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    wb.metadata_mut().user_defined.push(MetaUserDefined {
        name: "one".to_string(),
        value: MetaValue::String("val".to_string()),
    });
    wb.metadata_mut().user_defined.push(MetaUserDefined {
        name: "two".to_string(),
        value: MetaValue::TimeDuration(Duration::hours(0)),
    });
    wb.metadata_mut().user_defined.push(MetaUserDefined {
        name: "three".to_string(),
        value: MetaValue::Datetime(NaiveDateTime::default()),
    });
    wb.metadata_mut().user_defined.push(MetaUserDefined {
        name: "four".to_string(),
        value: MetaValue::Boolean(true),
    });
    wb.metadata_mut().user_defined.push(MetaUserDefined {
        name: "five".to_string(),
        value: MetaValue::Float(1.234),
    });

    let mut sh = Sheet::new("1");
    sh.set_value(0, 0, "A");
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_metadata.ods")?;

    let _wi = read_ods("test_out/test_metadata.ods")?;

    // dbg!(wi.metadata());

    Ok(())
}
