fn main() {
    let mut door_open = [false; 100];
    for pass in 1..100 {
        let mut door = pass;
        while door < 100 {
            door_open[door-1] = !door_open[door-1];
            door += pass;
        }
    }
    for(i, &is_open) in door_open.iter().enumerate(){
        println!("Door {} is {}", i, if is_open {"Open"} else {"Closed"});
    }
}
