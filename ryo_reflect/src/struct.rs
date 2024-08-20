use crate::reflect::Reflect;
use core::fmt::{Debug, Formatter};
use core::ops::{Index, IndexMut};

pub trait Struct: Reflect {
    fn as_struct(&self) -> &dyn Struct;
    fn as_struct_mut(&mut self) -> &mut dyn Struct;
    fn field(&self, name: &str) -> Option<&dyn Reflect>;
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect>;
    fn field_index(&self, index: usize) -> Option<&dyn Reflect>;
    fn field_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;
    fn field_count(&self) -> usize;
    fn field_name(&self, index: usize) -> Option<&'static str>;
    fn field_value(&self, index: usize) -> Option<&dyn Reflect>;
}

impl Index<usize> for dyn Struct {
    type Output = dyn Reflect;

    fn index(&self, index: usize) -> &Self::Output {
        self.field_index(index).expect("Out of bounds access")
    }
}

impl IndexMut<usize> for dyn Struct {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.field_index_mut(index).expect("Out of bounds access")
    }
}

impl Index<&str> for dyn Struct {
    type Output = dyn Reflect;

    fn index(&self, index: &str) -> &Self::Output {
        self.field(index).expect("Not a field")
    }
}

impl IndexMut<&str> for dyn Struct {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.field_mut(index).expect("Not a field")
    }
}

impl Debug for dyn Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct(self.type_name());

        (0..self.field_count()).into_iter().for_each(|index| {
            // SAFETY: index is in range
            let name = unsafe { self.field_name(index).unwrap_unchecked() };
            let value = unsafe { self.field_value(index).unwrap().as_any() };

            debug.field(name, &value as &dyn Debug);
        });

        debug.finish()
    }
}
