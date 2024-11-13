pub struct Page<T>
where
    T: Clone,
{
    pub items: Vec<T>,
    pub size: usize,
}

impl<T> Page<T>
where
    T: Clone,
{
    pub fn create(items: Vec<T>, size: usize) -> Self {
        Page { items, size }
    }
    pub fn nth(&self, number: usize) -> Vec<T> {
        if self.items.len() <= number * self.size {
            vec![]
        } else {
            self.items
                .iter()
                .skip(number * self.size)
                .take(self.size)
                .map(|item| item.clone())
                .collect::<Vec<T>>()
        }
    }
}
