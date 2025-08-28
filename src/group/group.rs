pub trait Group
where
    Self: Eq + Clone + Copy,
{
    fn identity() -> Self;
    fn op(self, rhs: Self) -> Self;
    fn inverse(self) -> Self;
}

impl Group for () {
    fn identity() -> Self {}
    fn op(self, rhs: Self) -> Self {}
    fn inverse(self) -> Self {}
}

impl Group for bool {
    fn identity() -> Self {
        false
    }

    fn op(self, rhs: Self) -> Self {
        match (self, rhs) {
            (false, false) => false,
            (false, true) => true,
            (true, false) => true,
            (true, true) => false,
        }
    }

    fn inverse(self) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ThreeGroup {
    E,
    A,
    B,
}

impl Group for ThreeGroup {
    fn identity() -> Self {
        Self::E
    }

    fn op(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::E, Self::E) => Self::E,
            (Self::E, Self::A) => Self::A,
            (Self::E, Self::B) => Self::B,

            (Self::A, Self::E) => Self::A,
            (Self::A, Self::A) => Self::B,
            (Self::A, Self::B) => Self::E,

            (Self::B, Self::E) => Self::B,
            (Self::B, Self::A) => Self::E,
            (Self::B, Self::B) => Self::A,
        }
    }

    fn inverse(self) -> Self {
        match self {
            Self::E => Self::E,
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Vierergruppe {
    E,
    A,
    B,
    C,
}

impl Group for Vierergruppe {
    fn identity() -> Self {
        Self::E
    }

    fn op(self, rhs: Self) -> Self {
        if matches!(self, Self::E) {
            return rhs;
        }
        if matches!(rhs, Self::E) {
            return self;
        }
        if self == rhs {
            return Self::E;
        }
        match (self, rhs) {
            (Self::A, Self::B) => Self::C,
            (Self::B, Self::A) => Self::C,
            (Self::A, Self::C) => Self::B,
            (Self::C, Self::A) => Self::B,
            (Self::B, Self::C) => Self::A,
            (Self::C, Self::B) => Self::A,
            _ => unreachable!(),
        }
    }

    fn inverse(self) -> Self {
        self
    }
}

impl Group for i8 {
    fn identity() -> Self {
        0
    }

    fn op(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    fn inverse(self) -> Self {
        self.wrapping_neg()
    }
}

#[cfg(test)]
mod tests {

    use super::{Group, ThreeGroup, Vierergruppe};

    pub trait IterateGroup
    where
        Self: Sized + Group,
    {
        fn all() -> Vec<Self>;
    }

    impl IterateGroup for bool {
        fn all() -> Vec<Self> {
            vec![false, true]
        }
    }

    impl IterateGroup for ThreeGroup {
        fn all() -> Vec<Self> {
            vec![ThreeGroup::E, ThreeGroup::A, ThreeGroup::B]
        }
    }

    impl IterateGroup for Vierergruppe {
        fn all() -> Vec<Self> {
            vec![
                Vierergruppe::E,
                Vierergruppe::A,
                Vierergruppe::B,
                Vierergruppe::C,
            ]
        }
    }

    impl IterateGroup for i8 {
        fn all() -> Vec<Self> {
            (i8::MIN..=i8::MAX).collect()
        }
    }

    fn group_test<G: IterateGroup + std::fmt::Debug>() {
        let elements = G::all();
        for g in elements {
            assert_eq!(G::identity().op(g), g);
            assert_eq!(g.op(G::identity()), g);
            assert_eq!(g.op(g.inverse()), G::identity());
            assert_eq!(g.inverse().op(g), G::identity());
        }
    }

    fn commutativity_test<G: IterateGroup + std::fmt::Debug>() {
        let elements = G::all();
        for a in elements.iter().cloned() {
            for b in elements.iter().cloned() {
                assert_eq!(a.op(b), b.op(a));
            }
        }
    }

    #[test]
    fn c2() {
        group_test::<bool>();
    }

    #[test]
    fn c2_size() {
        assert_eq!(2, bool::all().len())
    }

    #[test]
    fn c2_commutes() {
        commutativity_test::<bool>();
    }

    #[test]
    fn c3() {
        group_test::<ThreeGroup>();
    }

    #[test]
    fn c3_size() {
        assert_eq!(3, ThreeGroup::all().len())
    }

    #[test]
    fn c3_commutes() {
        commutativity_test::<ThreeGroup>();
    }

    #[test]
    fn vierergruppe() {
        group_test::<Vierergruppe>();
    }

    #[test]
    fn vierergruppe_size() {
        assert_eq!(4, Vierergruppe::all().len())
    }

    #[test]
    fn vierergruppe_commutes() {
        commutativity_test::<Vierergruppe>();
    }

    #[test]
    fn c256() {
        group_test::<i8>();
    }

    #[test]
    fn c256_size() {
        assert_eq!(256, i8::all().len())
    }

    #[test]
    fn c256_commutes() {
        commutativity_test::<i8>();
    }
}
