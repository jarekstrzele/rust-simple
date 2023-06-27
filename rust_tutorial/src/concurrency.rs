use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell ;
use std::sync::{Arc, Mutex} ;



pub fn tread_example(){
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap() ;

        if bank_ref.balance < 5.00 {
            println!("Current Balance: {} withdraw a smaollwe amount", bank_ref.balance) ;

        } else {
            bank_ref.balance -= amt ;
            println!("Customer withdrew {} Current balance {}", amt, bank_ref.balance) ;
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00) ;
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone() ;
        thread::spawn(|| {
            customer(bank_ref) ;
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance) ;
    



    // fn withdraw(the_bank: &mut Bank, amt: f32){
    //     the_bank.balance -= amt ;
    // }

    // let mut bank = Bank {balance: 100.0} ;
    // withdraw(&mut bank, 5.00) ;
    // println!("Balance : {} ", bank.balance) ;
    
    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00)
    // }

    // thread::spawn( || {
    //     customer(&mut bank)
    // }).join().unwrap();


}
