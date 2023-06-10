use std::collections::HashMap ;




fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);

    println!("Number of items: {:?}", stock.capacity());

    let mut total_stock = 0;
    for (name, number) in stock.iter() {
        total_stock += number;

        let stock_count = if *number == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", number)
        };

        println!("name: {:?}, number: {:?}", name, stock_count);
    }

    println!("total stock={:?}", total_stock);
}