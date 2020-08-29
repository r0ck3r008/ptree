struct Pnode<'a> {
    slice: &'a str,
    carr: [Child<'a>; 26],
}

type Child<'a> = Option<Box<Pnode<'a>>>;

struct Ptree<'a> {
    size: usize,
    root: Child<'a>,
}

fn hashit(s1: &str, s2: &str) -> usize {
    let mut count: usize = 0;
    let v1: Vec<_> = s1.chars().collect();
    let v2: Vec<_> = s2.chars().collect();
    while count < s1.len() {
        if &v1[count] == &v2[count] {
            count += 1;
        } else {
            break;
        }
    }

    return count;
}

impl<'a> Ptree<'a> {
    pub fn new() -> Self {
        return Self {
            size: 0,
            root: None,
        };
    }
}
