#[derive(PartialEq, Clone)]
pub struct CompiledFunction {
    name: String,
    head: parser::NodePtr,
    return_type: String,
    parameters: Vec<String>,
}

impl fmt::Debug for CompiledFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{} {}({}) at {:?}>",
            self.return_type,
            self.name,
            self.parameters.iter().map(ToString::to_string).join(","),
            self.head
        )
    }
}
