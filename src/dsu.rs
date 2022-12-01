#![allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct DSU <T, F> where T : Default, F : Fn(T, T) -> T {
    parents : Vec<usize>,
    data : Vec<T>,
    union_func : F,
    size : Vec<usize>,
}

impl <T, F> DSU<T, F> where T : Default, F : Fn(T, T) -> T {
    pub fn new(union_func : F) -> DSU<T, F> {
        DSU{parents : Vec::new(), data : Vec::new(), union_func, size : Vec::new()}
    }
    
    pub fn with_capacity(union_func : F, capacity : usize,) -> DSU<T, F> {
        DSU{parents : Vec::with_capacity(capacity), data : Vec::with_capacity(capacity), union_func, size : Vec::with_capacity(capacity)}
    }
    
    pub fn len(&self) -> usize {
        self.parents.len()
    }
    
    pub fn push(&mut self, data : T) -> usize {
        let len = self.len();
        self.parents.push(len);
        self.data.push(data);
        self.size.push(1);
        len
    }
    
    pub(crate) fn set_parent(&mut self, node : usize, parent : usize) {
        debug_assert!(node < self.len());
        self.parents[node] = parent
    }
    
    pub(crate) fn get_parent(&self, node : usize) -> usize {
        debug_assert!(node < self.len());
        self.parents[node]
    }
    
    pub(crate) fn set_data(&mut self, node : usize, data : T) {
        debug_assert!(node < self.len());
        self.data[node] = data;
    }
    
    pub fn get_data_mut(&mut self, node : usize) -> &mut T {
        debug_assert!(node < self.len());
        &mut self.data[node]
    }
    
    pub(crate) fn get_data(&self, node : usize) -> &T {
        debug_assert!(node < self.len());
        &self.data[node]
    }

    pub(crate) fn take_data(&mut self, node : usize) -> T {
        debug_assert!(node < self.len());
        std::mem::take(self.get_data_mut(node))
    }

    pub(crate) fn get_size(&self, node : usize) -> usize {
        debug_assert!(node < self.len());
        self.size[node]
    }

    pub(crate) fn set_size(&mut self, node : usize, size : usize) {
        debug_assert!(node < self.len());
        self.size[node] = size;
    }
    
    pub fn top(&self, node : usize) -> usize {
        if self.get_parent(node) == node {return node}
        self.top(self.get_parent(node))
    }
    
    pub fn top_mut(&mut self, node : usize) -> usize {
        if self.get_parent(node) == node {return node}
        let top = self.top(self.get_parent(node));
        
        self.set_parent(node, top);
        top
    }
    
    pub fn top_data(&self, node : usize) -> &T {
        self.get_data(self.top(node))
    }
    
    pub fn top_data_mut(&mut self, node : usize) -> &mut T{
        let top = self.top_mut(node);
        self.get_data_mut(top)
    }
    
    pub fn union(&mut self, node0 : usize, node1 : usize) { 
        let top0 = self.top_mut(node0);
        let top1 = self.top_mut(node1);
        
        let data0 = self.take_data(top0);
        let data1 = self.take_data(top1);

        let data_new = (self.union_func)(data0, data1);

        let size0 = self.get_size(top0);
        let size1 = self.get_size(top1);
        let size_new = size0 + size1;
        
        if size0 >= size1 {//TODO add with union + push a size aswell
            self.set_data(top0, data_new);
            self.set_size(top0, size_new);

            self.set_parent(top1, top0);
        }

        else {
            self.set_data(top1, data_new);
            self.set_size(top1, size_new);
            
            self.set_parent(top0, top1);
        }
    }
}
