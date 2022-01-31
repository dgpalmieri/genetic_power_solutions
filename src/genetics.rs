
pub struct Genetics {
    message: String,
}

impl Genetics {
    pub fn new(m: &str) -> Self {
        let message : String = m.to_string();
        return Self{ message };
    }
    pub fn hello(&mut self){
        println!("{}", self.message);
    }
}
