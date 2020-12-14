#[derive(PartialEq, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub head: parser::NodePtr,
    pub parameters: Vec<String>,
    pub return_type: String,
}

impl std::fmt::Debug for CompiledFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{} {}({}) at {:?}>",
            self.return_type,
            self.name,
            self.parameters.join(","),
            self.head
        )
    }
}
