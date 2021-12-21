pub struct DSU {
    parents : Vec<usize>,
}

impl DSU {
    pub fn new() -> DSU {
        DSU{parents : Vec::new()}
    }

    pub fn with_capacity(capacity : usize) -> DSU {
        DSU{parents : Vec::with_capacity(capacity)}
    }
    
    pub fn from(parents : Vec<usize>) -> DSU {
        DSU{parents : parents}
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn push(&mut self) -> usize {
        let len = self.parents.len();
        self.parents.push(len);
        len
    }

    pub fn append(&mut self, count : usize) {
        for _i in 0..count {
            self.push();
        }
    }

    pub(crate) fn set_parent(&mut self, node : usize, parent : usize) {
        debug_assert!(node < self.parents.len());
        self.parents[node] = parent
    }

    pub(crate) fn get_parent(&self, node : usize) -> usize {
        debug_assert!(node < self.parents.len());
        self.parents[node]
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

    pub fn union(&mut self, node0 : usize, node1 : usize) {
        let top0 = self.top_mut(node0);
        let top1 = self.top_mut(node1);

        self.set_parent(top1, top0);
    }
}