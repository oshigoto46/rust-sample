use std::fmt;

pub struct DataStore{
    hostname: String
}


impl  DataStore{
    //let mut hostname = "hoge.com";

    pub fn new() -> Self { 
        let mut hostname = String::from("hoge");
        DataStore{
            hostname:  hostname
        }
    }

}

