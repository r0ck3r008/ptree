use std::collections::HashMap;

struct Pnode {
    slice: String,
    hmap: HashMap<char, Child>,
}

type Child = Box<Pnode>;

struct Ptree {
    root: Child,
    size: usize,
}

fn getlvl(s1: &str, s2: &str) -> usize {
    let mut count = 0;
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();
    let mlen = if v1.len() < v2.len() {
        v1.len()
    } else {
        v2.len()
    };

    while count < mlen {
        if &v1[count] != &v2[count] {
            break;
        }
        count += 1;
    }

    return count;
}

impl Pnode {
    fn new(snew: &str) -> Self {
        return Self {
            slice: String::from(snew),
            hmap: HashMap::new(),
        };
    }

    fn update_hmap(&mut self, opt_snew: Option<&str>, lvl: usize, c: char) {
        let snew: &str = match opt_snew {
            None => &self.slice,
            Some(snew) => snew,
        };

        if let Some(node) = self.hmap.get_mut(&c) {
            node.add(&snew[lvl..]);
        } else {
            self.hmap.insert(c, Box::new(Pnode::new(&snew[lvl..])));
        }
    }

    fn add(&mut self, snew: &str) {
        let lvl: usize = getlvl(&self.slice, snew);
        if let Some(split) = snew.chars().nth(lvl) {
            self.update_hmap(Some(&snew), lvl, split);
        }

        if let Some(split) = self.slice.chars().nth(lvl) {
            self.update_hmap(None, lvl, split);
            self.slice = String::from(&self.slice[0..lvl]);
        }
    }

    fn find(&self, sfind: &str) -> bool {
        let lvl: usize = getlvl(&self.slice, sfind);
        match sfind.chars().nth(lvl) {
            None => {
                return true;
            }
            Some(split) => match self.hmap.get(&split) {
                None => {
                    return false;
                }
                Some(node) => {
                    return node.find(&sfind[lvl..]);
                }
            },
        };
    }
}

impl Ptree {
    pub fn new() -> Self {
        return Self {
            root: Box::new(Pnode::new("")),
            size: 0,
        };
    }

    pub fn add(&mut self, snew: &str) {
        self.root.add(snew);
    }

    pub fn find(&self, sfind: &str) -> bool {
        return self.root.find(sfind);
    }
}
