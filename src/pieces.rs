use std::collections::HashMap;
#[derive(Clone)]
enum Pieces {
    King(King),
    Queen(Queen),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Pawn(Pawn),
}

type PossibleMoves = HashMap<i8, Vec<i8>>;
#[derive(Clone)]
pub struct Board {
    board:Vec<Vec<Option<Pieces>>>,
    gameStatus:String,
}
impl Board {
    pub fn new() -> Self {
        let mut board:Vec<Vec<Option<Pieces>>>;
        board = vec![vec![None;8];8];
        for i in 0..7 {
            board[i][1] = Some(Pieces::Pawn(Pawn::new("White".to_string(),vec![i as i8,1 as i8])));
            board[i][6] = Some(Pieces::Pawn(Pawn::new("Black".to_string(),vec![i as i8,1 as i8])));
        };
        board[0][0] = Some(Pieces::Rook(Rook::new("White".to_string(), vec![0 as i8, 0 as i8])));
        board[7][0] = Some(Pieces::Rook(Rook::new("White".to_string(), vec![7 as i8, 0 as i8])));
        board[0][7] = Some(Pieces::Rook(Rook::new("Black".to_string(), vec![0 as i8, 7 as i8])));
        board[7][7] = Some(Pieces::Rook(Rook::new("Black".to_string(), vec![7 as i8, 7 as i8])));
        board[1][0] = Some(Pieces::Knight(Knight::new("White".to_string(), vec![1 as i8, 0 as i8])));
        board[6][0] = Some(Pieces::Knight(Knight::new("White".to_string(), vec![6 as i8, 0 as i8])));
        board[1][7] = Some(Pieces::Knight(Knight::new("Black".to_string(), vec![1 as i8, 7 as i8])));
        board[6][7] = Some(Pieces::Knight(Knight::new("Black".to_string(), vec![6 as i8, 7 as i8])));
        board[2][0] = Some(Pieces::Bishop(Bishop::new("White".to_string(), vec![2 as i8, 0 as i8])));
        board[5][0] = Some(Pieces::Bishop(Bishop::new("White".to_string(), vec![5 as i8, 0 as i8])));
        board[2][7] = Some(Pieces::Bishop(Bishop::new("Black".to_string(), vec![2 as i8, 7 as i8])));
        board[5][7] = Some(Pieces::Bishop(Bishop::new("Black".to_string(), vec![5 as i8, 7 as i8])));
        board[3][0] = Some(Pieces::Queen(Queen::new("White".to_string(), vec![3 as i8, 0 as i8])));
        board[4][0] = Some(Pieces::King(King::new("White".to_string(), vec![4 as i8, 0 as i8])));
        board[3][7] = Some(Pieces::Queen(Queen::new("Black".to_string(), vec![3 as i8, 7 as i8])));
        board[4][7] = Some(Pieces::King(King::new("Black".to_string(), vec![4 as i8, 7 as i8])));
        Self {
            board,
            gameStatus:"Ongoing".to_string(),
        }
    }
}
#[derive(Clone)]
pub struct King {
    position:Vec<i8>,
    inCheck:bool,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    ownBoard:Board,
    
}
impl King {
    pub fn get_moves() -> HashMap<i8, Vec<i8>> {
        let mut possibleMoves:PossibleMoves = Vec::new();
        let mut tracker:i8 = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue
                }
                tracker+=1;
                possibleMoves.insert(tracker, vec![i,j]);
            }
        }
        return possibleMoves;
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
           for j in 0..8 {
               // if board.
           } 
        }
        true

    }
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut position;
        let position = if color == "White" {
                position = vec![0 as i8,4 as i8]
            } else {
                position = vec![7 as i8,4 as i8]
            };
        Self {
            position: pos,
            inCheck:false,
            color,
            name:"King".to_string(),
            possibleMoves:Self::get_moves(),
            ownBoard: Board::new(),
        }
    }
}
#[derive(Clone)]
pub struct Queen {
    position:Vec<i8>,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    
}
impl Queen {
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut possibleMoves:Vec<Vec<i8>> = Vec::new();
        possibleMoves.push_back(vec![pos[0],pos[1]]);
        Self {
            position:pos,
            color,
            name: "Queen".to_string(),
            possibleMoves,
            
        }
    }
}
#[derive(Clone)]
pub struct Knight {
    position:Vec<i8>,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    
}
impl Knight {
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut possibleMoves:Vec<Vec<i8>> = Vec::new();
        possibleMoves.push_back(vec![pos[0],pos[1]]);
        Self {
            position:pos,
            color,
            name: "Knight".to_string(),
            
            possibleMoves,
            
        }
    }
}
#[derive(Clone)]
pub struct Bishop {
    position:Vec<i8>,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    
}
impl Bishop {
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut possibleMoves:Vec<Vec<i8>> = Vec::new();
        possibleMoves.push_back(vec![pos[0],pos[1]]);
        Self {
            position:pos,
            color,
            name: "Bishop".to_string(),
            
            possibleMoves,
            
        }
    }
}
#[derive(Clone)]
pub struct Rook {
    position:Vec<i8>,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    
}
impl Rook {
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut possibleMoves:Vec<Vec<i8>> = Vec::new();
        possibleMoves.push_back(vec![pos[0],pos[1]]);
        Self {
            position:pos,
            color,
            name: "Rook".to_string(),
            
            possibleMoves,
            
        }
    }
}
#[derive(Clone)]
pub struct Pawn {
    position:Vec<i8>,
    color:String,
    name:String,
    possibleMoves:Vec<i8>,
    enpassantPossible:bool,
    
}
impl Pawn {
    pub fn new(color:String, pos:Vec<i8>) -> Self {
        let mut possibleMoves:Vec<Vec<i8>> = Vec::new();
        Self {
            position:pos,
            color,
            name: "Pawn".to_string(),
            possibleMoves, 
            enpassantPossible:true,
        }
    }
}
