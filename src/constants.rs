pub const ROW_UP : isize = -1;
pub const ROW_DOWN : isize = 1;
pub const ROW_NEUTRAL : isize = 0;
pub const COL_LEFT : isize = -1;
pub const COL_RIGHT : isize = -1;
pub const COL_NEUTRAL : isize = 0;

pub const KNIGHT_OFFSETS : [(isize, isize); 8] = 
[
    (2,-1),(2,1),
    (1,2) ,(-1,2),
    (-2,1),(-2,-1),
    (1,-2),(-1,-2)
];



pub const DIRECTION_UP : (isize, isize) = (ROW_UP, COL_NEUTRAL);
pub const DIRECTION_UP_RIGHT : (isize, isize) = (ROW_UP, COL_RIGHT);
pub const DIRECTION_RIGHT : (isize, isize) = (ROW_NEUTRAL, COL_RIGHT);
pub const DIRECTION_DOWN_RIGHT : (isize, isize) = (ROW_DOWN, COL_RIGHT);
pub const DIRECTION_DOWN : (isize, isize) = (ROW_DOWN, COL_NEUTRAL);
pub const DIRECTION_DOWN_LEFT : (isize, isize) = (ROW_DOWN, COL_LEFT);
pub const DIRECTION_LEFT : (isize, isize) = (ROW_NEUTRAL, COL_LEFT);
pub const DIRECTION_UP_LEFT : (isize, isize) = (ROW_UP, COL_LEFT);




pub const BISHOP_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP,
    DIRECTION_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_LEFT,
];

pub const ROOK_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN_LEFT,
    DIRECTION_UP_LEFT
];

pub const QUEEN_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP,
    DIRECTION_UP_RIGHT,
    DIRECTION_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_DOWN_LEFT,
    DIRECTION_LEFT,
    DIRECTION_UP_LEFT
];

pub const KING_OFFSETS : [(isize,isize);8] = 
[
    DIRECTION_UP,
    DIRECTION_UP_RIGHT,
    DIRECTION_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_DOWN_LEFT,
    DIRECTION_LEFT,
    DIRECTION_UP_LEFT
];
