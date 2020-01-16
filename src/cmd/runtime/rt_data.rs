pub struct Slot {
    num: u32,
}
pub struct OperdStack {
    size: u32,
    slots: Vec<Slot>,
}
pub struct Frame {
    next: &Frame,
    local_vars: Vec<Slot>,
    operdStack: &OperdStack,
}
pub struct Stack {
    max_size: u32,
    size: u32,
    top: &Frame,
}
pub struct Thread {
    pc: u32,
    stack: &Stack,
}

impl Thread {
    pub fn setPc(&mut self, pc: u32) {
        self.pc = pc;
    }
}

impl Stack {
    pub fn push(&mut self, frame: &Frame) {
        if slef.size == self.max_size {
            panic!("");
        }
        frame.next = self.top;
        self.top = frame;
        self.size++;
    }

    pub fn pop(&mut self) -> Frame {
        if self.size == 0 {
            panic!("");
        }
        let pframe = *self.top;
        self.top = self.top.next;
        self.size--;
        pframe
    }

    pub fn top(&mut self) -> Frame {
        if self.size == 0 {
            panic!("");
        }
        *self.top 
    }
}
