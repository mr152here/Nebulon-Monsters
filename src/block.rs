use glam::Vec2;


//cover is a group of the blocks. Each is destroyed individualy
pub struct Block {
    position: Vec2
}

impl Block {
    pub fn new(position: Vec2) -> Block {
        Block { position }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}


pub struct BlockGroup {
    blocks: Vec<Option<Block>>,
}

impl BlockGroup {

    pub fn new() -> BlockGroup {
        BlockGroup {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(Some(block));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Block>> {
        self.blocks.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<Block>> {
        self.blocks.iter_mut()
    }
}
