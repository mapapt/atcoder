use std::{ops::Index, rc::Rc};

//#############################################################################

#[derive(Debug)]
struct SkipNode<T>
{
    value: Option<Rc<T>>,
    width: usize,
    next: Option<usize>,
    down: Option<usize>,
}

#[derive(Debug)]
struct SkipList<T>
{
    max_level: usize,
    nodes: Vec<SkipNode<T>>,
    free_head: Option<usize>,
    rnd_st: u64,
}

impl<T> SkipList<T>
{
    fn new() -> SkipList<T> {
        let max_level = 32;
        let mut nodes = Vec::new();
        for i in 0..=max_level {
            nodes.push(SkipNode {
                value: None,
                width: 0,
                next: None,
                down: if i < max_level {Some(i + 1)} else {None},
            });
        }
        SkipList {
            max_level: max_level,
            nodes: nodes,
            free_head: None,
            rnd_st: 88172645463325252,
        }
    }
    fn len(&self) -> usize {
        let mut c_id = 0;
        let mut sum_width = 0;
        while let Some(r_id) = self.nodes[c_id].next {
            sum_width += self.nodes[c_id].width;
            c_id = r_id;
        }
        sum_width += self.nodes[c_id].width;
        sum_width
    }
}

impl<T: Ord> SkipList<T>
{
    fn h_xorshift64(state: u64) -> u64 {
        let state = state ^ (state << 13);
        let state = state ^ (state >> 7);
        let state = state ^ (state << 17);
        state
    }
    fn h_insert(&mut self, index: usize, value: Rc<T>, level: usize) -> (Option<usize>, usize) {
        //println!("{} {:?} {}", index, self.nodes[&index].value, level);
        let mut c_id = index;
        let mut sum_width = 0;
        while let Some(r_id) = self.nodes[c_id].next {
            let right = &self.nodes[r_id];
            if &value >= right.value.as_ref().unwrap() {
                sum_width += self.nodes[c_id].width;
                c_id = r_id;
            }
            else {
                break;
            }
        }

        let (a_id, a_width) = if let Some(b_id) = self.nodes[c_id].down {
            self.h_insert(b_id, value.clone(), level.max(1) - 1)
        }
        else {
            (None, 1)
        };

        if level == 0 {
            let n_node = SkipNode {
                value: Some(value),
                width: self.nodes[c_id].width + 1 - a_width,
                next: self.nodes[c_id].next,
                down: a_id,
            };

            let n_id = if let Some(f_idx) = self.free_head {
                self.free_head = self.nodes[f_idx].next;
                self.nodes[f_idx] = n_node;
                f_idx
            }
            else {
                self.nodes.push(n_node);
                self.nodes.len() - 1
            };

            self.nodes[c_id].next = Some(n_id);
            self.nodes[c_id].width = a_width;

            //println!("---");
            //println!("{}: {:?}", c_id, cur_mut);
            //println!("{}: {:?}", self.max_index, self.nodes[&self.max_index]);
            //println!("{} {}", sum_width, a_width);

            (Some(n_id), sum_width + a_width)
        }
        else {
            self.nodes[c_id].width += 1;

            //println!("{:?}", cur_mut);
            //println!("extend {}", sum_width + a_width);

            (None, sum_width + a_width)
        }
    }
    //
    fn insert(&mut self, value: T) -> usize {
        self.rnd_st = Self::h_xorshift64(self.rnd_st);

        let level = self.rnd_st.trailing_zeros() as usize;

        //println!("{:?} {}", value, level);
        let (_, width) = self.h_insert(0, Rc::new(value), self.max_level.max(level) - level);
        width - 1
    }
}

impl<T: Ord> SkipList<T>
{
    fn search(&self, x: &T) -> Result<usize, usize> {
        let mut c_id = 0;
        let mut sum_width = 0;
        loop {
            //println!("{}", c_id);
            if let Some(r_id) = self.nodes[c_id].next {
                let right = &self.nodes[r_id];
                if x >= right.value.as_ref().unwrap() {
                    sum_width += self.nodes[c_id].width;
                    c_id = r_id;
                    continue;
                }
            }

            if let Some(b_id) = self.nodes[c_id].down {
                c_id = b_id;
                continue;
            }
            else {
                break;
            }
        }

        //println!("{}", c_id);
        if let Some(y) = &self.nodes[c_id].value {
            if x == y.as_ref() {
                return Ok(sum_width - 1);
            }
        }
        Err(sum_width)
    }
}

impl<T> Index<usize> for SkipList<T>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut c_id = 0;
        let mut sum_width = 0;
        loop {
            //println!("{}", c_id);
            if index + 1 == sum_width {
                return self.nodes[c_id].value.as_ref().unwrap();
            }
            else {
                if index + 1 >= sum_width + self.nodes[c_id].width {
                    if let Some(r_id) = self.nodes[c_id].next {
                        sum_width += self.nodes[c_id].width;
                        c_id = r_id;
                        continue;
                    }
                }
                if let Some(b_id) = self.nodes[c_id].down {
                    c_id = b_id;
                    continue;
                }
                panic!("Index:{} not found", index);
            }
        }
    }
}

struct SkipListIter<'a, T>
{
    nodes: &'a[SkipNode<T>],
    c_id: usize,
}

impl<'a, T> Iterator for SkipListIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(n_id) = self.nodes[self.c_id].next {
            self.c_id = n_id;
            Some(self.nodes[self.c_id].value.as_ref().unwrap())
        }
        else {
            None
        }
    }
}

impl<T> SkipList<T>
{
    fn iter(&self) -> SkipListIter<'_, T> {
        SkipListIter {
            nodes: &self.nodes,
            c_id: self.max_level,
        }
    }
}

// fn remove(&mut self, index: usize) -> Option<T>
// fn remove<Q>(&mut self, value: &Q) -> bool
// Debug
// fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>

//#############################################################################

#[test]
fn test_skip_list() {
    let mut s: SkipList<u32> = SkipList::new();
    assert_eq!(s.insert(10), 0); // 10
    assert_eq!(s.insert(20), 1); // 10,20
    assert_eq!(s.insert(30), 2); // 10,20,30
    assert_eq!(s.insert(40), 3); // 10,20,30,40
    assert_eq!(s.insert(50), 4); // 10,20,30,40,50
    assert_eq!(s.len(), 5);
    assert_eq!(s.search(&5), Err(0));
    assert_eq!(s.search(&10), Ok(0));
    assert_eq!(s.search(&25), Err(2));
    assert_eq!(s.search(&50), Ok(4));
    assert_eq!(s.search(&55), Err(5));
    assert_eq!(s[0], 10);
    assert_eq!(s[1], 20);
    assert_eq!(s[2], 30);
    assert_eq!(s[3], 40);
    assert_eq!(s[4], 50);
    for (i, &v) in s.iter().enumerate() {
        assert_eq!(v, (i + 1) as u32 * 10);
    }
}
