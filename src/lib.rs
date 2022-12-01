pub mod dsu;
pub use dsu::*;


#[cfg(test)]
mod tests {
    use crate::dsu::*;


    #[test]
    fn add() {
        let mut  dsu : DSU<i32, _> = DSU::new(|x, y| x + y);

        for i in 0..10 {
            dsu.push(i);
        }

        dsu.union(0, 1);
        dsu.union(2, 3);
        dsu.union(1, 2);

        assert_eq!(dsu.top_mut(0), dsu.top_mut(3));
        assert_ne!(dsu.top_mut(1), dsu.top_mut(4));

        assert_eq!(dsu.top_data(0), &6);
        assert_eq!(dsu.top_data(1), &6);
        assert_eq!(dsu.top_data(2), &6);
        assert_eq!(dsu.top_data(3), &6);
    }

    #[test]
    fn max() {
        let mut  dsu : DSU<i32, _> = DSU::new(|x, y| std::cmp::max(x, y));

        for i in 0..10 {
            dsu.push(i*2);
        }

        dsu.union(0, 1);
        dsu.union(2, 9);
        dsu.union(1, 2);

        assert_eq!(dsu.top_mut(0), dsu.top_mut(9));
        assert_ne!(dsu.top_mut(1), dsu.top_mut(4));

        assert_eq!(dsu.top_data(0), &18);
        assert_eq!(dsu.top_data(1), &18);
        assert_eq!(dsu.top_data(2), &18);
        assert_eq!(dsu.top_data(3), &6);
    }
}
