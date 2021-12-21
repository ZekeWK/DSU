mod dsu;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::dsu::*;


    #[test]
    fn union() {
        let mut  dsu = DSU::new();

        dsu.append(10);

        dsu.union(0, 1);
        dsu.union(1, 2);
        dsu.union(2, 3);

        assert_eq!(dsu.top_mut(0), dsu.top_mut(3));
        assert_ne!(dsu.top_mut(1), dsu.top_mut(4));
    }

}