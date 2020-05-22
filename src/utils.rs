pub fn union<T: PartialEq + Clone>(vector1: &[T], vector2: &[T]) -> Vec<T>
{
    let mut unioned: Vec<T> = vec!();
    for element in vector1 {
        unioned.push(element.clone());
    }
    for element in vector2 {
        if !unioned.contains(element) {
            unioned.push(element.clone());
        }
    }
    unioned
}

pub fn intersect<T: PartialEq + Clone>(vector1: &[T], vector2: &[T]) -> Vec<T>
{
    let mut intersected = vec!();
    for element in vector1 {
        if vector2.contains(element) {
            intersected.push(element.clone());
        }
    }
    intersected
}

// pub fn outersect<T: PartialEq + Clone>(vector1: &Vec<T>, vector2: &Vec<T>) -> Vec<T>
// {
//     let mut outersected = vec!();
//     for element in vector1 {
//         if !vector2.contains(element) {
//             outersected.push(element.clone());
//         }
//     }
//     for element in vector2 {
//         if !vector1.contains(element) {
//             outersected.push(element.clone());
//         }
//     }
//     return outersected;
// }

pub fn outersect_left<T: PartialEq + Clone>(vector1: &[T], vector2: &[T]) -> Vec<T>
{
    let mut outersected = vec!();
    for element in vector1 {
        if !vector2.contains(element) {
            outersected.push(element.clone());
        }
    }
    outersected
}