static MAX_ENTITIES: u16 = u16::MAX;

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentVec for std::cell::RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub struct World {
    component_vector: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_vector: Vec::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity_id: u16, component: T) where T: Copy {
        if let Some(storage) = self.find_storage_for_type::<T>() {
            storage[usize::from(entity_id)] = Some(component);
        }

        // allocate once, for the whole lifetime of the program.
        let mut storage_vector = vec![None; usize::from(MAX_ENTITIES)];
        storage_vector[usize::from(entity_id)] = Some(component);
        self.component_vector.push(Box::new(std::cell::RefCell::new(storage_vector)));
    }

    pub fn delete_component<T: 'static>(&mut self, entity_id: u16) {
        if let Some(storage) = self.find_storage_for_type::<T>() {
            storage[usize::from(entity_id)] = None;
        }
    }

    pub fn get_component_from_entity<T: 'static>(&mut self, entity_id: u16) -> Option<T> where T: Copy {
        if let Some(storage) = self.find_storage_for_type::<T>() {
            return storage[usize::from(entity_id)];
        }
        None
    }

    pub fn borrow_component_vec<T: 'static>(&self) -> Vec<std::cell::RefMut<Vec<Option<T>>>> {
        let mut result: Vec<std::cell::RefMut<Vec<Option<T>>>> = Vec::new();
        for storage_vector in self.component_vector.iter() {
            if let Some(storage) = storage_vector
                .as_any()
                .downcast_ref::<std::cell::RefCell<Vec<Option<T>>>>() {
                result.push(storage.borrow_mut());
            }
        }
        result
    }

    fn find_storage_for_type<T: 'static>(&mut self) -> Option<&mut Vec<Option<T>>> {
        for storage_vector in self.component_vector.iter_mut() {
            if let Some(storage) = storage_vector
                .as_any_mut()
                .downcast_mut::<std::cell::RefCell<Vec<Option<T>>>>() {
                return Some(storage.get_mut());
            }
        }
        None
    }
}

#[macro_export]
macro_rules! declare_system {
    ($macro_name:ident|$($name:ident),+) => {
        paste! {
            fn [<$macro_name>]<F>(
                world: &World,
                updater: F
            ) where F: FnMut(($(&mut $name),+)) -> () {
                izip!(
                    $(world.borrow_component_vec::<$name>()
                        .iter_mut()
                        .map(|x|x.iter_mut())
                        .flatten()
                    ),+
                )
                .filter_map(|($([<$name:lower>]),+)| {
                    Some(($([<$name:lower>].as_mut()?),+))
                })
                .for_each(updater);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //#[test]
    //fn it_works() {
    //    assert_eq!(2, 4);
    //}
}
