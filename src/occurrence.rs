pub enum Occurrence {
    Multiple,
    Single(usize),
    None,
}

impl Occurrence {
    pub fn from<T, P>(iterable: &[T], predicate: P) -> Occurrence
        where P: Fn(&T) -> bool
    {
        let pos = iterable.iter().position(&predicate);
        let rpos = iterable.iter().rposition(&predicate);

        if let Some(pos_value) = pos {
            if pos != rpos {
                Occurrence::Multiple
            } else {
                Occurrence::Single(pos_value)
            }
        } else {
            Occurrence::None
        }
    }

    pub fn is_multiple(&self) -> bool {
        if let Occurrence::Multiple = self {
            true
        } else {
            false
        }
    }

    pub fn is_single(&self) -> bool {
        if let Occurrence::Single(_) = self {
            true
        } else {
            false
        }
    }

    pub fn unwrap(&self) -> usize {
        match self {
            Occurrence::Single(pos) => *pos,
            Occurrence::Multiple => panic!("Called `Occurrence::unwrap()` on a `Multiple` value"),
            Occurrence::None => panic!("Called `Occurrence::unwrap()` on a `None` value"),
        }
    }
}
