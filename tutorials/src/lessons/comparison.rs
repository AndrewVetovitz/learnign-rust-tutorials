#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if equal(_first_list, _second_list) { return Comparison::Equal; }
    if subset(_first_list, _second_list) { return Comparison::Sublist; }
    if superset(_first_list, _second_list) { return Comparison::Superlist; }

    return Comparison::Unequal;
}

fn equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    return a == b;
}

fn subset<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 {
        return true;
    }

    if a.len() > b.len() {
        return false;
    }

    for i in 0..(b.len() - a.len() + 1) {
        if a[0] == b[i] && equal(a, &b[i..i+a.len()])  {
            return true;
        }
    }

    return false;
}

fn superset<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    return subset(b, a);
}