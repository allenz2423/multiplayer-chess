use std::collections::HashMap;
#[derive(Clone)]
enum Pieces {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
    Empty,
}

type PossibleMoves = HashMap<i8, Vec<(i8,i8)>>;

pub struct Board {
    board:Vec<Vec<Pieces>>,
    gameStatus:String,
}
impl Board {
    pub fn new() -> Self {
        Self {
            board:vec![vec![Pieces::Empty;8];8],
            gameStatus:"Ongoing".to_string(),
        }
    }
}

pub struct King {
    position:Vec<i8>,
    inCheck:bool,
    color:String,
    name:String,
    possibleMoves:HashMap<i8, Vec<(i8, i8)>>,
    board:Board,
}
impl King {
    pub fn get_moves() {
        let mut possibleMoves:PossibleMoves = HashMap::new();
        let mut tracker:i8 = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue
                }
                tracker+=1;
                possibleMoves.insert(tracker, vec![(i,j)]);
            }
        }
        possibleMoves;
    }
    pub fn is_valid_move(&self, otherKing:King) -> bool {
        // You have to check if you are within 2 units of distance of the other King
        let distanceX:f64 = (self.position[0] as f64 - otherKing.position[0] as f64).powf(2 as f64);
        let distanceY:f64 = (self.position[1] as f64 - otherKing.position[1] as f64).powf(2 as f64);
        let distance:f64 = (distanceX+distanceY).sqrt();
        if distance < 2.0 {
            return false
        }
        for i in 0..8 {
            
        }
        true

    }
    pub fn new(&self,color:String) -> Self {
        let mut position;
        let position = if color == "White" {
                position = vec![0 as i8,4 as i8]
            } else {
                position = vec![7 as i8,4 as i8]
            };
        Self {
            position: self.position.clone(),
            inCheck:false,
            color,
            name:"King".to_string(),
            possibleMoves: self.possibleMoves.clone(),
            board: Board::new(),
        }
    }
}
pub struct Queen {

}
pub struct Knight {

}
pub struct Bishop {

}
pub struct Rook {

}
pub struct Pawn {

}
