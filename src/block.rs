pub const BLK: Block = Block {
    kind: BlockKind::Basic,
    color: 0xA00000,
};

pub const AIR: Block = Block {
    kind: BlockKind::None,
    color: 0x000000,
};

pub enum BlockKind {
    Basic,
    None,
}

pub struct Block {
    kind: BlockKind,
    color: u32,
}
