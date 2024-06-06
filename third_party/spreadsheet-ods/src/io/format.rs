use crate::validation::Validation;
use chrono::Duration;
use std::fmt::{Display, Formatter};

pub(crate) fn format_duration2(v: Duration) -> impl Display {
    struct Tmp(Duration);

    impl Display for Tmp {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "PT{}H{}M{}.{}S",
                self.0.num_hours(),
                self.0.num_minutes() % 60,
                self.0.num_seconds() % 60,
                self.0.num_milliseconds() % 1000
            )
        }
    }

    Tmp(v)
}

pub(crate) fn format_validation_condition(v: &Validation) -> impl Display + '_ {
    struct Tmp<'f>(&'f Validation);

    impl<'f> Display for Tmp<'f> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "of:{}", self.0.condition())
        }
    }

    Tmp(v)
}
