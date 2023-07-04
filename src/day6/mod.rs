use std::fs;

/*
    start packet signal => 4 different char sequence
    
    intermediate assignment => find starter packet
    final question => amount of chars before starter packet
*/
/*
    PART 2  
    The same but packet size is 14
*/

const STARTER_PACKET_SIZE: usize = 14;

pub fn run(){
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    let signal_strings = input.lines();

    for signal in signal_strings {
        let signal_chars = signal.chars();
        
        for (i, _) in signal_chars.enumerate().skip(STARTER_PACKET_SIZE-1) {
            let packet = &signal[i-(STARTER_PACKET_SIZE-1)..i+1];
            let mut some_char_repeated = false;

            for char in packet.chars() {
                let char_count = packet.matches(char).count();
                if char_count > 1 {
                    some_char_repeated = true;
                    break;
                }
            }

            if !some_char_repeated {
                println!("Starter packet: {} - Index of packet: {}", packet , i+1);
                break;
            }
        }

        println!()
    }
}