use crate::compiler::Compiler;

impl Compiler {
    pub fn parse_loop_statement(&mut self) {
        let prev_loop_start = self.loop_start;
        let prev_loop_depth = self.loop_depth;
        self.loop_start = Some(self.current_chunk().bytecodes.len());
        self.loop_depth = self.scope_depth;

        self.parse_statement();
        self.emit_loop(self.loop_start.unwrap());

        self.end_loop();
        self.loop_start = prev_loop_start;
        self.loop_depth = prev_loop_depth;
    }
}
