use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id<T: Clone>(usize, PhantomData<T>);

impl<T> PartialEq for Id<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Storage<T: Clone> {
    storage: Vec<T>,
}

impl<T> Storage<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Storage { storage: vec![] }
    }

    pub fn add(&mut self, value: T) -> Id<T> {
        self.storage.push(value);
        Id(self.storage.len() - 1, PhantomData::default())
    }

    // this would require invalidating every Id instance
    // pub fn remove(&mut self, id: Id<T>) -> T {
    //     self.storage.remove(id.0)
    // }

    pub fn get(&self, id: &Id<T>) -> &T {
        // assume the id is valid
        self.storage.get(id.0).unwrap()
    }

    pub fn get_mut(&mut self, id: &Id<T>) -> &mut T {
        // assume the id is valid
        self.storage.get_mut(id.0).unwrap()
    }

    pub fn as_vec(&self) -> Vec<(Id<T>, &T)> {
        let mut vec = vec![];
        for (id, value) in self.storage.iter().enumerate() {
            vec.push((Id(id, PhantomData::default()), value));
        }
        vec
    }

    pub fn as_mut_vec(&mut self) -> Vec<(Id<T>, &mut T)> {
        let mut vec = vec![];
        for (id, value) in self.storage.iter_mut().enumerate() {
            vec.push((Id(id, PhantomData::default()), value));
        }
        vec
    }
}

impl<T> Default for Storage<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Copy for Id<T> where T: Clone {}
