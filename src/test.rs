use crate::game::*;

#[test]
fn test_win_conditions() -> (){
    static TRUTH_TABLE : [[Option<bool>; Choice::LENGTH]; Choice::LENGTH] = [
        [None,          Some(true),     Some(false)],
        [Some(false),   None,           Some(true)],
        [Some(true),    Some(false),    None],
    ];
    
    for i in 0..TRUTH_TABLE.len() {
        for j in 0..TRUTH_TABLE.len() {
            let condition_string = conditions_to_string(TRUTH_TABLE[i][j]);

            match (Choice::try_from(i as u8), Choice::try_from(j as u8)) {
                (Ok(me), Ok(you)) => {
                    let game_outcome = me.get_outcome(you);
                    println!("i: {0}, j: {1}, val: {2}, outcome: {3} ",
                        i,
                        j,
                        condition_string,
                        conditions_to_string(game_outcome),
                    );
                    
                    assert_eq!(game_outcome, TRUTH_TABLE[i][j]);
                }
                _ => panic!("hjølp"),
            }
        }
    }
}

fn conditions_to_string(wld : Option<bool>) -> &'static str {
    match wld {
        Some(true)  =>  "Win :-)",
        None        =>  "Draw :-|",
        Some(false) =>  "Loss :-(",
    }
}