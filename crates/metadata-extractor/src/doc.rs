use syn::{visit::Visit, Lit};

#[derive(Default, Debug)]
pub struct Docs(Vec<String>);

impl<'ast> Visit<'ast> for Docs {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path().is_ident("doc") {
            if let Ok(Lit::Str(content)) = i.parse_args::<Lit>() {
                self.0.push(content.value());
            }
        }
    }
}

impl Docs {
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}
