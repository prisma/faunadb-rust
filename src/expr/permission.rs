use crate::expr::{Expr, Ref};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum SimpleLevel {
    #[serde(rename = "public")]
    Public,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AnnotatedLevel<'a> {
    Reference(Expr<'a>)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Level<'a> {
    Annotated(AnnotatedLevel<'a>),
    Simple(SimpleLevel)
}

impl<'a> Level<'a> {
    pub fn reference(reference: Ref<'a>) -> Self {
        Level::Annotated(AnnotatedLevel::Reference(Expr::from(reference)))
    }

    pub fn public() -> Self {
        Level::Simple(SimpleLevel::Public)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClassPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Creating an instance in the class.
    create: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reading instances in the class.
    read: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Writing to instances in the class.
    write: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClassPermission<'a> {
    object: ClassPermissionObject<'a>
}

impl<'a> ClassPermission<'a> {
    pub fn create(&mut self, level: Level<'a>) -> &mut Self {
        self.object.create = Some(level);
        self
    }

    pub fn read(&mut self, level: Level<'a>) -> &mut Self {
        self.object.read = Some(level);
        self
    }

    pub fn write(&mut self, level: Level<'a>) -> &mut Self {
        self.object.write = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstancePermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reading this instance
    read: Option<Level<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Writing to this instance
    write: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstancePermission<'a> {
    object: InstancePermissionObject<'a>,
}

impl<'a> InstancePermission<'a> {
    pub fn read(&mut self, level: Level<'a>) -> &mut Self {
        self.object.read = Some(level);
        self
    }

    pub fn write(&mut self, level: Level<'a>) -> &mut Self {
        self.object.write = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FunctionPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Calling the function
    call: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FunctionPermission<'a> {
    object: FunctionPermissionObject<'a>,
}

impl<'a> FunctionPermission<'a> {
    pub fn call(&mut self, level: Level<'a>) -> &mut Self {
        self.object.call = Some(level);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IndexPermissionObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Querying the index
    read: Option<Level<'a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IndexPermission<'a> {
    object: IndexPermissionObject<'a>,
}

impl<'a> IndexPermission<'a> {
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

        assert_eq!(
            expected,
            serde_json::to_value(&perm).unwrap(),
        )
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

        assert_eq!(
            expected,
            serde_json::to_value(&perm).unwrap(),
        )
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

        assert_eq!(
            expected,
            serde_json::to_value(&perm).unwrap(),
        )
    }
}
