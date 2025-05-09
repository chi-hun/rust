pub trait Summary {
    //fn sumarize(&self) -> String;

    fn sumarize(&self) -> String {
        format!("fffff")
    }
}

pub struct News {
    pub title : String,
    pub main : String,
    pub who : String,
}

impl Summary for News {
    fn sumarize(&self) -> String {
        format!("title : {}, main : {}, who : {}", self.title, self.main, self.who)
    }
}

pub struct Tweet {
    pub title : String,
    pub hash : String,
    pub who : String,
}

impl Summary for Tweet {
}
