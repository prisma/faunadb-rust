//! Authentication functions
use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query![HasIdentity, Identify, Identity, Login, Logout];

/// The `HasIdentity` function returns `true` if the current client
/// authentication credentials have an associated identity, and `false` if they
/// don’t.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/authentication/hasidentity).
#[derive(Serialize, Clone, Debug, Default)]
pub struct HasIdentity<'a> {
    has_identity: Expr<'a>,
}

impl<'a> HasIdentity<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

/// The `Identify` function checks the given password against the ref’s
/// credentials, returning `true` if the credentials are valid, or false
/// otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/authentication/identify).
#[derive(Serialize, Clone, Debug)]
pub struct Identify<'a> {
    identify: Expr<'a>,
    password: Expr<'a>,
}

impl<'a> Identify<'a> {
    pub fn new(identify: Ref<'a>, password: impl Into<Expr<'a>>) -> Self {
        Self {
            identify: Expr::from(identify),
            password: password.into(),
        }
    }
}

/// The `Identity` function returns the ref of the instance associated with the
/// authentication token used for the request. If an instance does not exist, an
/// error is returned.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/authentication/identity).
#[derive(Serialize, Clone, Debug, Default)]
pub struct Identity<'a> {
    identity: Expr<'a>,
}

impl<'a> Identity<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

/// The `Login` function creates an authentication token for the provided Ref, or
/// Set of Refs.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/authentication/login).
#[derive(Serialize, Clone, Debug)]
pub struct Login<'a> {
    login: Expr<'a>,
    params: LoginParams<'a>,
}

#[derive(Serialize, Clone, Debug)]
#[doc(hidden)]
pub struct LoginObject<'a> {
    password: Expr<'a>,
}

#[derive(Serialize, Clone, Debug)]
#[doc(hidden)]
pub struct LoginParams<'a> {
    object: LoginObject<'a>,
}

impl<'a> Login<'a> {
    pub fn new(login: Ref<'a>, password: impl Into<Expr<'a>>) -> Self {
        Self {
            login: Expr::from(login),
            params: LoginParams {
                object: LoginObject {
                    password: password.into(),
                },
            },
        }
    }
}

/// The `Logout` function deletes all tokens associated with the current session
/// if its parameter is `true`, or just the token used in this request otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/authentication/logout).
#[derive(Serialize, Clone, Debug)]
pub struct Logout<'a> {
    logout: Expr<'a>,
}

impl<'a> Logout<'a> {
    pub fn new(all_tokens: bool) -> Self {
        Self {
            logout: Expr::from(all_tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_has_identity() {
        let fun = HasIdentity::new();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({ "has_identity": null }), serialized);
    }

    #[test]
    fn test_identity() {
        let fun = Identity::new();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({ "identity": null }), serialized);
    }

    #[test]
    fn test_identify() {
        let mut user_ref = Ref::instance("1234");
        user_ref.set_class("characters");

        let fun = Identify::new(user_ref, "Hunter2");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "identify": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "class": {
                                "@ref": {
                                    "id": "classes"
                                }
                            },
                            "id": "characters"
                        }
                    },
                    "id": "1234"
                }
            },
            "password": "Hunter2",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_login() {
        let mut user_ref = Ref::instance("1234");
        user_ref.set_class("characters");

        let fun = Login::new(user_ref, "Hunter2");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "login": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "class": {
                                "@ref": {
                                    "id": "classes"
                                }
                            },
                            "id": "characters"
                        }
                    },
                    "id": "1234"
                }
            },
            "params": {
                "object": {
                    "password": "Hunter2"
                }
            },
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_logout() {
        let fun = Logout::new(false);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "logout": false,
        });

        assert_eq!(expected, serialized);
    }
}
