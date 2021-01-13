fn main() {

    let params = vec![123,23456,6788];
    match sum(&params){
        Some(v)=>println!("total value is {}", v),
        None =>println!("overflow")
    }
}

fn sum(numbers:&[u32])->Option<u32>{
    let mut total:u32 = 0;
    for val in numbers.iter(){
        match total.checked_add(*val){
            Some(v)=>{total = v},
            None=>{return None}
        }
    }
    Some(total)
}