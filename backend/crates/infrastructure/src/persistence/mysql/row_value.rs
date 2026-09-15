#[macro_export]
macro_rules! mysql_row_value {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
        #[sqlx(transparent)]
        pub struct $name($inner);

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}
