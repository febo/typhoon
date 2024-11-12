use syn::visit::Visit;

#[derive(Default)]
pub struct Constraints(Vec<String>);

impl<'ast> Visit<'ast> for Constraints {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        // i.
    }
}

pub enum ConstraintList {
    Init,
    ATA,
}
