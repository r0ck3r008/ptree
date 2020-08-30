use std::collections::HashMap;

struct Pnode<'a> {
    slice: &'a str,
    hmap: HashMap<char, Child<'a>>,
}

type Child<'a> = Box<Pnode<'a>>;

struct Ptree<'a> {
    root: Child<'a>,
    size: usize,
}

fn countlvl(s1: &str, s2: &str) -> usize {
    if &s1 == &"" {
        return 0;
    }
    let mut count: usize = 0;
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

impl<'a> Pnode<'a> {
    fn new(snew: &'a str) -> Self {
        return Self {
            slice: snew,
            hmap: HashMap::new(),
        };
    }

    fn add_to_hmap(&mut self, snew: &'a str, lvl: usize, splitat: char) {
        if let Some(node) = self.hmap.get_mut(&splitat) {
            node.add(&snew[lvl..]);
        } else {
            self.hmap
                .insert(splitat, Box::new(Pnode::new(&snew[lvl..])));
        }
    }

    fn add(&mut self, snew: &'a str) {
        let lvl = countlvl(self.slice, snew);
        match snew.chars().nth(lvl) {
            None => return,
            Some(split) => {
                self.add_to_hmap(snew, lvl, split);
            }
        };
        match self.slice.chars().nth(lvl) {
            None => return,
            Some(split) => {
                self.add_to_hmap(self.slice, lvl, split);
                self.slice = &self.slice[0..lvl];
            }
        };
    }

    fn find(&self, sfind: &str) -> bool {
        let lvl = countlvl(self.slice, sfind);
        match sfind.chars().nth(lvl) {
            None => {
                return true;
            }
            Some(c) => {
                if let Some(node) = self.hmap.get(&c) {
                    return node.find(&sfind[lvl..]);
                } else {
                    return false;
                }
            }
        }
    }
}

impl<'a> Ptree<'a> {
    pub fn new() -> Self {
        return Self {
            root: Box::new(Pnode::new("")),
            size: 0,
        };
    }

    pub fn add(&mut self, snew: &'a str) {
        self.root.add(snew);
        self.size += 1;
    }

    pub fn find(&self, sfind: &str) -> bool {
        return self.root.find(sfind);
    }
}
