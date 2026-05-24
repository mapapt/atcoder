use std::{collections::HashMap};

//#############################################################################

#[derive(Debug)]
struct SkipNode<T>
{
    value: Option<T>,
    width: usize,
    next: Option<usize>,
    down: Option<usize>,
}

#[derive(Debug)]
struct SkipList<T>
{
    max_level: usize,
    nodes: HashMap<usize, SkipNode<T>>,
    max_index: usize,
    rnd_st: u64,
}

impl<T: Ord + Copy + std::fmt::Debug> SkipList<T>
{
    fn h_xorshift64(state: u64) -> u64 {
        let state = state ^ (state << 13);
        let state = state ^ (state >> 7);
        let state = state ^ (state << 17);
        state
    }
    fn h_insert(&mut self, index: usize, value: T, level: usize) -> (Option<usize>, usize) {
        //println!("{} {:?} {}", index, self.nodes[&index].value, level);
        let mut c_id = index;
        let mut sum_width = 0;
        loop {
            if let Some(r_id) = self.nodes[&c_id].next {
                let right = &self.nodes[&r_id];
                if value >= right.value.unwrap() {
                    sum_width += self.nodes[&c_id].width;
                    c_id = r_id;
                }
            }
            break;
        }

        let (a_id, a_width) = if let Some(b_id) = self.nodes[&c_id].down {
            self.h_insert(b_id, value, level.max(1) - 1)
        }
        else {
            (None, 1)
        };

        if level == 0 {
            let cur = &self.nodes[&c_id];
            let n_next = cur.next;
            let n_width = cur.width + 1 - a_width;

            self.max_index += 1;
            self.nodes.insert(self.max_index, SkipNode {
                value: Some(value),
                width: n_width,
                next: n_next,
                down: a_id,
            });

            let cur_mut = self.nodes.get_mut(&c_id).unwrap();
            cur_mut.next = Some(self.max_index);
            cur_mut.width = a_width;

            //println!("---");
            //println!("{}: {:?}", c_id, cur_mut);
            //println!("{}: {:?}", self.max_index, self.nodes[&self.max_index]);
            //println!("{} {}", sum_width, a_width);

            (Some(self.max_index), sum_width + a_width)
        }
        else {
            let cur_mut = self.nodes.get_mut(&c_id).unwrap();
            cur_mut.width += 1;

            //println!("{:?}", cur_mut);
            //println!("extend {}", sum_width + a_width);

            (None, sum_width + a_width)
        }
    }
    //
    fn new() -> SkipList<T> {
        let max_level = 32;
        let mut nodes = HashMap::new();
        for i in 0..=max_level {
            nodes.insert(i, SkipNode {
                value: None,
                width: 0,
                next: None,
                down: if i < max_level {Some(i + 1)} else {None},
            });
        }
        SkipList {
            max_level: max_level,
            nodes: nodes,
            max_index: max_level,
            rnd_st: 88172645463325252,
        }
    }
    fn insert(&mut self, value: T) -> usize {
        self.rnd_st = Self::h_xorshift64(self.rnd_st);

        let level = self.rnd_st.trailing_zeros() as usize;

        //println!("{:?} {}", value, level);
        let (_, width) = self.h_insert(0, value, self.max_level.max(level) - level);
        width - 1
    }
    // fn contains(&self, value: &T) -> Option<usize>
    // fn remove(&mut self, index: usize) -> T
    // fn len(&self) -> usize
    // Debug
    // fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    // Index
    // fn index(&self, index: I) -> &<Vec<T, A> as Index<I>>::Output

    // fn clear(&mut self)
    // fn iter(&self) -> Iter<'_, T>
    // fn partition_point<P>(&self, pred: P) -> usize
    //   where P: FnMut(&T) -> bool
}

//#############################################################################

#[test]
fn test_skip_list() {
    let mut s: SkipList<u32> = SkipList::new();
    assert_eq!(s.insert(10), 0); // 10
    assert_eq!(s.insert(20), 1); // 10,20
    assert_eq!(s.insert(30), 2); // 10,20,30
    assert_eq!(s.insert(35), 3);
    //assert_eq!(s.insert(15), 1);
}
