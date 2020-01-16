
fn main() {
   
   pub mod movies {
    pub mod comedy {
       pub fn play(name:String) {
          println!("Playing comedy movie {}",name);
            }
        }
    }

use movies::comedy::play; 

play("HOUSEFULL 4".to_string());

}