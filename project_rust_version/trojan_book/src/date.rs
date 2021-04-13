


pub struct Date {
    day: usize,
    month: usize,
    year: usize 
}



impl Date {

    pub fn new() -> Self { // Default "constructor"

        Date {
            day: 28,
            month: 3,
            year: 1999 
        }
    }

    pub fn print_date (&self, typ: &str) {

        let months =["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

        if typ == "M/D/YYYY" {
            print!("{}/{}/{}", self.month, self.day, self.year);
        }

        else if typ == "Month D, YYYY"{
            println!("{} {}, {}", months[self.month-1], self.day, self.year);
        }

        else if typ == "D-Month-YYYY"{
            println!("{}-{}-{}", self.day, months[self.month -1], self.year);
        }

        else{
            println!("Wrong print type!");
        }
    }

    pub fn set_date (s: &str) -> bool {
        let (mut day_, mut month_, mut yr_) = ("", "","");
        let mut state: u8 = 0; 

        let count = s.chars().count();
        
        for i in 0..count{
            if s.chars().nth(i).unwrap() == '/'{
                state +=1;
                continue;
            }

            if state == 0 {
                month_.a

            }
        }


        return true;

    }

}
