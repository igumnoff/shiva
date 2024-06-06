/// deg angles. 360°
#[macro_export]
macro_rules! deg {
    ($l:expr) => {
        Angle::Deg($l.into()).into()
    };
}

/// grad angles. 400°
#[macro_export]
macro_rules! grad {
    ($l:expr) => {
        Angle::Grad($l.into()).into()
    };
}

/// radians angle.
#[macro_export]
macro_rules! rad {
    ($l:expr) => {
        Angle::Rad($l.into()).into()
    };
}

/// Centimeters.
#[macro_export]
macro_rules! cm {
    ($l:expr) => {
        Length::Cm($l.into()).into()
    };
}

/// Millimeters.
#[macro_export]
macro_rules! mm {
    ($l:expr) => {
        Length::Mm($l.into()).into()
    };
}

/// Inches.
#[macro_export]
macro_rules! inch {
    ($l:expr) => {
        Length::In($l.into()).into()
    };
}

/// Point. 1/72"
#[macro_export]
macro_rules! pt {
    ($l:expr) => {
        Length::Pt($l.into()).into()
    };
}

/// Pica. 12/72"
#[macro_export]
macro_rules! pc {
    ($l:expr) => {
        Length::Pc($l into()).into()
    };
}

/// Length depending on font size.
#[macro_export]
macro_rules! em {
    ($l:expr) => {
        Length::Em($l into()).into()
    };
}
