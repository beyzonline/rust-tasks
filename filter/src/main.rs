// fn main() {
//     println!("Hello, world!");
// }

struct FilterCondition<T>{
    filter: T,
}

impl<T: PartialOrd> FilterCondition<T>{
    fn is_match(&self, item: &T) -> bool {
        item > &self.filter
    }
}

fn custom_filter<T>(list: Vec<T> , condition: &FilterCondition<T>) -> Vec<T> where T: PartialOrd {
    return list.into_iter().filter(|item: &T| condition.is_match(item)).collect();
}

fn main(){
    let numbers: Vec<i32> = vec![5,6,10,15,20];
    let condition: FilterCondition<i32> = FilterCondition { filter: 8 };
    
    let filtered_list: Vec<i32> = custom_filter(numbers, &condition);
    println!("{:?}" , filtered_list);
}