pub struct DSU <T> where T : std::cmp::PartialOrd{
    parents : Vec<usize>,
    data : Vec<T>
}

impl <T> DSU<T> where T : std::cmp::PartialOrd {
    pub fn new() -> DSU<T> {
        DSU{parents : Vec::new(), data : Vec::new()}
    }

    pub fn with_capacity(capacity : usize) -> DSU<T> {
        DSU{parents : Vec::with_capacity(capacity), data : Vec::with_capacity(capacity)}
    }
    
    pub fn from(parents : Vec<usize>, data : Vec<T>) -> DSU<T> {
        DSU{parents : parents, data : data}
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn push(&mut self, data : T) -> usize {
        let len = self.len();
        self.parents.push(len);
        self.data.push(data);
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
        self.data[node] = data;
    }

    pub fn get_data_mut(&mut self, node : usize) -> &mut T {
        &mut self.data[node]
    }

    pub(crate) fn get_data(&self, node : usize) -> &T {
        &self.data[node]
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

        let data0 = self.get_data(node0);
        let data1 = self.get_data(node1);
        
        if data0 >= data1 {
            self.set_parent(top1, top0);
        }
        else {
            self.set_parent(top0, top1);
        }
    }
}