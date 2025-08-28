use std::collections::{HashSet, VecDeque};

#[derive(Default)]
struct GroupPresentation {
    symbols: Vec<char>,
    rules: Vec<(String, String)>,
}

struct Group {
    symbols: Vec<char>,
    rules: Vec<(String, String)>,
}

// NOTE(lubo): Something of this sort! :)
struct GroupElement {
    id: usize,
    name: Vec<String>,
    aliases: Vec<String>,
    left_compose: HashMap<usize, usize>,
}

impl GroupPresentation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_symbol(mut self, symbol: char) -> Self {
        self.symbols.push(symbol);
        self
    }
    pub fn with_equality(mut self, lhs: String, rhs: String) -> Self {
        self.rules.push((lhs, rhs));
        self
    }
    pub fn build(self) -> Group {
        Group {
            symbols: self.symbols,
            rules: self.rules,
        }
    }
}

impl Group {
    fn simplify(&self, mut g: String) -> String {
        // println!("simplifying {:?}", g);
        for _ in 0..10 {
            let mut simplified = true;
            for rule in self.rules.iter() {
                let applied = g.replace(&rule.0, &rule.1);
                if g != applied {
                    g = applied;
                    // println!("applied {:?}", rule);
                    // println!("{:?}", g);
                    simplified = false;
                }
            }
            if simplified {
                return g;
            }
        }
        panic!("Simplify loop limit reached")
    }

    pub fn find_all_elements(&self) -> HashSet<String> {
        let mut closed = HashSet::new();
        let mut open = VecDeque::from([String::new()]);

        let mut limit = 50;
        while let Some(word) = open.pop_front() {
            // if word.len() > 1 {
            //     open.push_back(self.simplify(word[1..].into()));
            //     open.push_back(self.simplify(word[..word.len()].into()));
            // }

            for c in self.symbols.iter() {
                let child = format!("{}{}", word, c);
                let child = self.simplify(child);
                open.push_back(child);
            }
            closed.insert(word);

            println!("closed: {:?}", closed);

            limit -= 1;
            if limit <= 0 {
                panic!("graph search loop limit reached")
            }
        }

        closed
    }
}

#[cfg(test)]
mod tests {
    use super::GroupPresentation;

    #[test]
    fn vierergruppe() {
        let group = GroupPresentation::new()
            .with_symbol('a')
            .with_symbol('b')
            .with_equality("aa".into(), "".into())
            .with_equality("bb".into(), "".into())
            .with_equality("abab".into(), "".into())
            .build();

        dbg!(group.find_all_elements());
    }
}
