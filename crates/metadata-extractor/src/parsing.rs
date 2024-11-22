use syn::{Expr, File, Ident, Item, ItemFn, Stmt, Type};

#[derive(Default, Debug)]
pub struct ParsingContext<'a> {
    pub contexts: Vec<&'a Ident>,
    pub instructions: Vec<&'a Ident>,
    pub accounts: Vec<&'a Ident>,
}

impl<'a> From<&'a File> for ParsingContext<'a> {
    fn from(value: &'a File) -> Self {
        let mut context = ParsingContext::default();
        value.items.iter().for_each(|item| match item {
            Item::Impl(item_impl) => {
                if let Some(ident) = extract_ident(item_impl, "HandlerContext") {
                    context.contexts.push(ident);
                }

                if let Some(ident) = extract_ident(item_impl, "Owner") {
                    context.accounts.push(ident);
                }
            }
            Item::Fn(item_fn) => {
                if let Some(instructions) = extract_instruction_idents(item_fn) {
                    context.instructions = instructions;
                }
            }
            _ => (),
        });
        context
    }
}

fn extract_ident<'a>(item_impl: &'a syn::ItemImpl, trait_name: &str) -> Option<&'a Ident> {
    let trait_ = item_impl.trait_.as_ref()?;
    let segment = trait_.1.segments.last()?;

    if segment.ident != trait_name {
        return None;
    }

    match *item_impl.self_ty {
        Type::Path(ref type_path) => Some(&type_path.path.segments.last()?.ident),
        _ => None,
    }
}

fn extract_instruction_idents(item_fn: &ItemFn) -> Option<Vec<&Ident>> {
    // Check if it's the process_instruction function
    if item_fn.sig.ident != "process_instruction" {
        return None;
    }

    // Find match expression in function body
    let match_expr = item_fn.block.stmts.iter().find_map(|stmt| {
        if let Stmt::Expr(Expr::Match(m), ..) = stmt {
            Some(m)
        } else {
            None
        }
    })?;

    // Extract instruction identifiers from match arms
    let instructions = match_expr
        .arms
        .iter()
        .filter_map(|arm| {
            // Look for try expressions containing handle calls
            let Expr::Try(try_expr) = arm.body.as_ref() else {
                return None;
            };

            // Extract handle call
            let Expr::Call(call) = try_expr.expr.as_ref() else {
                return None;
            };

            // Verify it's a handle function
            let Expr::Path(p) = call.func.as_ref() else {
                return None;
            };
            if p.path.segments.last()?.ident != "handle" {
                return None;
            };

            // Get instruction identifier from last argument
            call.args.last().and_then(|arg| {
                if let Expr::Path(p) = arg {
                    p.path.get_ident()
                } else {
                    None
                }
            })
        })
        .collect();

    Some(instructions)
}
