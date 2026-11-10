/*
By: Aum markandey
Date: 2026-11-10
Program Details: <Program Description Here>
*/
use std::io;

fn input() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
     let inputnum = input.trim().parse().expect("Please type a number!");
    inputnum
}
fn main() {
    let mut starplayers = 0;
    let mut goldteam = false;
  let players = input();
      for i in 0..players {
        let points = input();
        let fouls = input();
        let total_points = points*5-fouls*3;
        if total_points>40{
            starplayers+=1;
        }
      }
   
   

if starplayers== players{
    goldteam = true;
}

if goldteam{
println!("You typed: {}+", starplayers);
} else {
    println!("You typed: {}", starplayers);
}

}
