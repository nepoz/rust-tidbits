// inefficient; might be making big clones of heap data
pub fn find_largest<T> (list : &[T]) -> T
    where T: PartialOrd + Clone
{
    let mut largest = list[0].clone();

    for i in list {
        if i.clone() > largest {largest = i.clone();}
    }
    largest.clone()
}

// Trying to avoid Cloning by using refs, but need indexing :(
// Note that this works, but is not great for non-latin UTF-8 encoding
pub fn find_biggest<T: PartialOrd> (list: &[T]) -> &T {
    let mut biggest: &T = &list[0..1][0];

    for i in list {
        if i > biggest {biggest = &i;}
    }

    biggest
}
