use std::marker::PhantomData;

use super::common::Id;
use super::environ::Environ;
use super::operation::{Op, OpId};

pub trait Entity: Id {
    type DataTypeT;
    fn get_dtype(&self) -> Option<Self::DataTypeT>;

    fn get_def<E: Environ>(&self, env: &E) -> Option<OpId>;
    fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId>;
    fn as_id(&self) -> EntityId;
    fn get_parent(&self) -> Option<RegionId>;
    fn set_parent(&mut self, parent: RegionId);
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct EntityId(pub usize);

impl From<usize> for EntityId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl Id for EntityId {
    fn id(&self) -> usize {
        self.0
    }
    fn set_id(&mut self, id: usize) {
        self.0 = id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    id: usize,
    op_children: Vec<OpId>,
    entity_children: Vec<EntityId>,
}

impl Id for Region {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegionId(pub usize);
impl Id for RegionId {
    fn id(&self) -> usize {
        self.0
    }

    fn set_id(&mut self, id: usize) {
        self.0 = id
    }
}

impl Region {

    pub fn get_use<E: Environ>(&self, env: &E) -> Option<OpId> {
        env.get_region_use(self.as_id())
    }

    pub fn as_id(&self) -> RegionId {
        RegionId(self.id)
    }

    pub fn new() -> Self {
        Self {
            id: 0,
            op_children: vec![],
            entity_children: vec![],
        }
    }

    pub fn add_op_child(&mut self, op: OpId) {
        if let Some(_) = self.op_children.iter().find(|&op_exist| op_exist.id() == op.id()) {
            panic!("{} has already been in the op_children of {}", op.id(), self.id())
        } else {
            self.op_children.push(op)
        }
    }

    pub fn add_entity_child(&mut self, entity: EntityId) {
        if let Some(_) = self.entity_children.iter().find(|&entity_exist| entity_exist.id() == entity.id()) {
            panic!("{} has already been in the entity_children of {}", entity.id(), self.id())
        } else {
            self.entity_children.push(entity)
        }
    }
}

#[macro_export]
macro_rules! entity_def {
    ([data_type = $data_type:ty] $name_enum:ident = {$($name:ident $(: (has_data=$expr:ident))?),+}) => {
        $(irony::entity_def_one! {
            $name : ($(has_data = $expr,)? data_type = $data_type)
        })*

        irony::entity_enum! {
            [data_type = $data_type]
            $name_enum = $($name),*
        }
    };
}

#[macro_export]
macro_rules! entity_def_one {
    ($name:ident : (data_type = $data_type:ty)) => {
        irony::entity_def_one! {
            $name : (has_data = false, data_type = $data_type)
        }
    };

    ($name:ident : (has_data = true, data_type = $data_type:ty)) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            pub id: usize,
            pub sym: irony::Symbol,
            pub parent: Option<irony::RegionId>,
            pub dtype: Option<$data_type>,
        }

        impl irony::Entity for $name {
            type DataTypeT = $data_type;

            fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
                env.get_def(self.as_id())
            }

            fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                env.get_uses(self.as_id())
            }

            fn get_dtype(&self) -> Option<Self::DataTypeT> {
                self.dtype
            }

            fn as_id(&self) -> irony::EntityId {
                irony::EntityId(self.id)
            }

            fn get_parent(&self) -> Option<irony::RegionId> {
                self.parent
            }

            fn set_parent(&mut self, parent: irony::RegionId) {
                self.parent = Some(parent)
            }
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                self.id
            }
            fn set_id(&mut self, id: usize) {
                self.id = id
            }
        }

        impl $name {
            pub fn new(name: &str, dtype: $data_type) -> Self {
                Self {
                    id: 0,
                    sym: irony::Symbol::new(String::from(name)),
                    dtype: Some(dtype),
                    parent: None,
                }
            }
        }
    };

    ($name:ident : (has_data = false, data_type = $data_type:ty)) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            pub id: usize,
            pub sym: irony::Symbol,
            pub parent: Option<irony::RegionId>,
            pub dtype: Option<$data_type>,
        }

        impl irony::Entity for $name {
            type DataTypeT = $data_type;

            fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
                env.get_def(self.as_id())
            }

            fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                env.get_uses(self.as_id())
            }

            fn get_dtype(&self) -> Option<Self::DataTypeT> {
                self.dtype
            }

            fn as_id(&self) -> irony::EntityId {
                irony::EntityId(self.id)
            }

            fn get_parent(&self) -> Option<irony::RegionId> {
                self.parent
            }

            fn set_parent(&mut self, parent: irony::RegionId) {
                self.parent = Some(parent)
            }
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                self.id
            }
            fn set_id(&mut self, id: usize) {
                self.id = id
            }
        }

        impl $name {
            pub fn new(name: &str) -> Self {
                Self {
                    id: 0,
                    sym: irony::Symbol::new(String::from(name)),
                    dtype: None,
                    parent: None,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! entity_enum {
    ([data_type = $dtype:ty] $name:ident= $($variant:ident),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variant($variant)),*
        }

        impl irony::Entity for $name {
            type DataTypeT = $dtype;

            fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
                match self {
                    $($name::$variant(inner) => inner.get_def(env),)*
                }
            }

            fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                match self {
                    $($name::$variant(inner) => inner.get_uses(env),)*
                }
            }

            fn get_dtype(&self) -> Option<Self::DataTypeT> {
                match self {
                    $($name::$variant(inner) => inner.get_dtype(),)*
                }
            }

            fn as_id(&self) -> irony::EntityId {
                match self {
                    $($name::$variant(inner) => inner.as_id(), )*
                }
            }

            fn get_parent(&self) -> Option<irony::RegionId> {
                match self {
                    $($name::$variant(inner) => inner.get_parent(), )*
                }
            }

            fn set_parent(&mut self, parent: irony::RegionId) {
                match self {
                    $($name::$variant(inner) => inner.set_parent(parent), )*
                }
            }
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                match self {
                    $($name::$variant(inner) => inner.id(), )*
                }
            }
            fn set_id(&mut self, id: usize) {
                match self {
                    $($name::$variant(inner) => inner.set_id(id), )*
                }
            }
        }

        $(
            impl Into<$name> for $variant {
                fn into(self) -> $name {
                    $name::$variant(self)
                }
            }
        )*

        $(
            impl Into<$variant> for $name {
                fn into(self) -> $variant {
                    match self {
                        $name::$variant(inner) => inner,
                        _ => panic!("match fails, check variant {} and enum {}", stringify!($variant), stringify!($name))
                    }
                }
            }
        )*


    };
}