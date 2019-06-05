/// A helper macro to implement `From` trait from the given query type into the
/// `Query` enum.
#[macro_export]
macro_rules! query {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind<'a>> for Query<'a> {
                fn from(q: $kind<'a>) -> Self {
                    Query::$kind(q)
                }
            }
        )*
    );
}

/// A helper macro to implement `From` trait from the given query type into the
/// `Query` enum, boxing the query.
#[macro_export]
macro_rules! boxed_query {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind<'a>> for Query<'a> {
                fn from(q: $kind<'a>) -> Self {
                    Query::$kind(Box::new(q))
                }
            }
        )*
    );
}

/// A convenience to convert a type of a signed integer into Fauna `Expr`.
#[macro_export]
macro_rules! int_expr {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind> for Number {
                fn from(i: $kind) -> Number {
                    Number::Int(i64::from(i))
                }
            }

            impl<'a> From<$kind> for Expr<'a> {
                fn from(i: $kind) -> Expr<'a> {
                    Expr::Simple(SimpleExpr::Number(i.into()))
                }
            }
        )*
    );
}

/// A convenience to convert a type of a unsigned integer into Fauna `Expr`.
#[macro_export]
macro_rules! uint_expr {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind> for Number {
                fn from(i: $kind) -> Number {
                    Number::UInt(u64::from(i))
                }
            }

            impl<'a> From<$kind> for Expr<'a> {
                fn from(i: $kind) -> Expr<'a> {
                    Expr::Simple(SimpleExpr::Number(i.into()))
                }
            }
        )*
    );
}
