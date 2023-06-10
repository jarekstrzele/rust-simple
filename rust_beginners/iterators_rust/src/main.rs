


fn main() {
   
   // filter borrows `n` so you have to borrow 10 ->>> &10
    let data:Vec<_> = vec![1,2,3,4,5].iter().map(|n| n*3).filter(|n| *n> 10).collect() ;
    // or  filter(|n| n>&10)


    for n in data{
        println!("{}", n) ;
    }
}
