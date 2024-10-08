use crate::reflect::Reflect;
use core::ops::{Index, IndexMut};

pub trait Enum: Reflect {
    fn as_enum(&self) -> &dyn Enum;
    fn as_enum_mut(&self) -> &mut dyn Enum;

    fn variant(&self, name: &str) -> Option<&dyn Reflect>;
}

impl Index<usize> for dyn Enum {
    type Output = dyn Reflect;

    fn index(&self, _index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for dyn Enum {
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        todo!()
    }
}
