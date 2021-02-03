use crate::ir::*;

pub fn check_lifetime(module: &Module) -> Module {
    Module::new(
        module.variable_declarations().to_vec(),
        module.function_declarations().to_vec(),
        vec![],
        vec![],
    )
}
