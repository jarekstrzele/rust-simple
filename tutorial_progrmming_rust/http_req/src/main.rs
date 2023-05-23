use reqwest ;


fn main() {
    let response_text = reqwest::blocking::get("http://youtube.local/hello")
        .expect("Couldn't make request") 
        .text()
        .expect("Could not read response text!") ;

    println!("Response text: {}" , response_text) ;

}
