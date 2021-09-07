#[derive(Debug, Clone)]
pub struct BlockSides {
    pub top: bool,
    pub bot: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

impl Default for BlockSides {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockSides {
    pub fn new() -> BlockSides {
        BlockSides {
            top: false,
            bot: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }
    pub fn set_all(&mut self, b: bool) {
        self.top = b;
        self.bot = b;
        self.left = b;
        self.right = b;
        self.front = b;
        self.back = b;
    }
    pub fn is_all(&mut self, b: bool) -> bool {
        self.left == b
            && self.right == b
            && self.back == b
            && self.bot == b
            && self.front == b
            && self.top == b
    }
}
