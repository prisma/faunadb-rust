use crate::expr::{Expr, Ref};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[doc(hidden)]
pub enum SimpleLevel {
    #[serde(rename = "public")]
    Public,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
#[doc(hidden)]
pub enum AnnotatedLevel<'a> {
    Reference(Expr<'a>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
/// Permission level definition.
pub enum Level<'a> {
    Annotated(AnnotatedLevel<'a>),
    Simple(SimpleLevel),
}

impl<'a> Level<'a> {
    /// Only tokens belonging to the specified Ref are allowed.
    ///
    /// Can refer to a:
    ///
    /// - Class: Only tokens belonging to instances in the specified class are allowed.
    /// - Instance: Only tokens belonging to the specified instance are allowed.
    pub fn reference(reference: Ref<'a>) -> Self {
        Level::Annotated(AnnotatedLevel::Reference(Expr::from(reference)))
    }

    /// Any key is allowed.
    pub fn public() -> Self {
        Level::Simple(SimpleLevel::Public)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[doc(hidden)]
pub struct ClassPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
/// Creating, reading, and modifying an instance in a class is controlled by the
/// class’s permissions.
///
/// See the [docs](https://docs.fauna.com/fauna/current/reference/security#class-permissions)
pub struct ClassPermission<'a> {
    object: ClassPermissionObject<'a>,
}

impl<'a> ClassPermission<'a> {
    /// Creating an instance in the class.
    pub fn create(&mut self, level: Level<'a>) -> &mut Self {
        self.object.create = Some(level);
        self
    }

    /// Reading instances in the class.
    pub fn read(&mut self, level: Level<'a>) -> &mut Self {
        self.object.read = Some(level);
        self
    }

    /// Writing to instances in the class.
    pub fn write(&mut self, level: Level<'a>) -> &mut Self {
        self.object.write = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[doc(hidden)]
pub struct InstancePermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reading this instance.
    read: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Writing to this instance.
    write: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// An instance also has permissions, which are applied in addition to
/// permissions defined on its class.
///
/// See the [docs](https://docs.fauna.com/fauna/current/reference/security#instance-permissions)
pub struct InstancePermission<'a> {
    object: InstancePermissionObject<'a>,
}

impl<'a> InstancePermission<'a> {
    /// Reading this instance.
    pub fn read(&mut self, level: Level<'a>) -> &mut Self {
        self.object.read = Some(level);
        self
    }

    /// Writing to this instance.
    pub fn write(&mut self, level: Level<'a>) -> &mut Self {
        self.object.write = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[doc(hidden)]
pub struct FunctionPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Calling the function
    call: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Calling a function is controlled by its permissions.
///
/// See the [docs](https://docs.fauna.com/fauna/current/reference/security#instance-permissions)
pub struct FunctionPermission<'a> {
    object: FunctionPermissionObject<'a>,
}

impl<'a> FunctionPermission<'a> {
    /// Calling the function.
    pub fn call(&mut self, level: Level<'a>) -> &mut Self {
        self.object.call = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[doc(hidden)]
pub struct IndexPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Querying the index
    read: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Query access to an index is controlled by its permissions.
///
/// See the [docs](https://docs.fauna.com/fauna/current/reference/security#instance-permissions)
pub struct IndexPermission<'a> {
    object: IndexPermissionObject<'a>,
}

impl<'a> IndexPermission<'a> {
    /// Querying the index.
    pub fn read(&mut self, level: Level<'a>) -> &mut Self {
        self.object.read = Some(level);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_class_permission() {
        let mut instance_ref = Ref::instance("musti");
        instance_ref.set_class("HouseCats");

        let class_ref = Ref::class("HouseCats");

        let mut perm = ClassPermission::default();
        perm.create(Level::public());
        perm.read(Level::reference(instance_ref));
        perm.write(Level::reference(class_ref));

        let expected = json!({
            "object": {
                "create": "public",
                "read": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "class": {
                                    "@ref": {
                                        "id": "classes"
                                    }
                                },
                                "id": "HouseCats"
                            }
                        },
                        "id": "musti"
                    }
                },
                "write": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "id": "classes"
                            }
                        },
                        "id": "HouseCats"
                    },
                },
            }
        });

        assert_eq!(expected, serde_json::to_value(&perm).unwrap(),)
    }

    #[test]
    fn test_instance_permission() {
        let mut instance_ref = Ref::instance("musti");
        instance_ref.set_class("HouseCats");

        let class_ref = Ref::class("HouseCats");

        let mut perm = InstancePermission::default();
        perm.read(Level::reference(instance_ref));
        perm.write(Level::reference(class_ref));

        let expected = json!({
            "object": {
                "read": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "class": {
                                    "@ref": {
                                        "id": "classes"
                                    }
                                },
                                "id": "HouseCats"
                            }
                        },
                        "id": "musti"
                    }
                },
                "write": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "id": "classes"
                            }
                        },
                        "id": "HouseCats"
                    },
                },
            }
        });

        assert_eq!(expected, serde_json::to_value(&perm).unwrap(),)
    }

    #[test]
    fn test_function_permission() {
        let mut perm = FunctionPermission::default();
        perm.call(Level::public());

        assert_eq!(
            json!({ "object": {"call": "public"} }),
            serde_json::to_value(&perm).unwrap(),
        )
    }

    #[test]
    fn test_index_permission() {
        let class_ref = Ref::class("HouseCats");

        let mut perm = IndexPermission::default();
        perm.read(Level::reference(class_ref));

        let expected = json!({
            "object": {
                "read": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "id": "classes"
                            }
                        },
                        "id": "HouseCats"
                    },
                },
            }
        });

        assert_eq!(expected, serde_json::to_value(&perm).unwrap(),)
    }
}
