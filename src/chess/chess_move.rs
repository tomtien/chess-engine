use bitflags::bitflags;

const START_BITMASK: u16 = 0b0000000000111111;
const END_BITMASK: u16 = 0b0000111111000000;
const FLAG_BITMASK: u16 = 0b1111000000000000;

const START_OFFSET: u16 = 0;
const END_OFFSET: u16 = 6;
const FLAG_OFFSET: u16 = 12;

bitflags! {
    pub struct  MoveFlags:u16 {
        const EN_PASSANT = 1;
        const CASTLE_SHORT = 2;
        const CASTLE_LONG = 3;
        const PROMOTE_QUEEN = 4;
        const PROMOTE_ROOK =5;
        const PROMOTE_BISHOP = 6;
        const PROMOTE_KNIGHT = 7;
        const PAWN_TWO_FORWARD = 8;
    }
}

#[derive(Debug)]
pub struct ChessMove {
    pub data: u16,
}
impl ChessMove {
    pub fn new(start_pos: u16, end_pos: u16, flags: MoveFlags) -> Self {
        Self {
            data: start_pos << START_OFFSET | end_pos << END_OFFSET | flags.bits() << FLAG_OFFSET,
        }
    }
    #[inline(always)]
    pub fn get_idx(&self) -> (u16, u16, MoveFlags) {
        let start_idx = (self.data & START_BITMASK) >> START_OFFSET;
        let end_idx = (self.data & END_BITMASK) >> END_OFFSET;

        let flags = MoveFlags::from_bits_truncate((self.data & FLAG_BITMASK) >> FLAG_OFFSET); // maybe unchecked
        return (start_idx, end_idx, flags);
    }
}
