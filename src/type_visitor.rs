use syn::spanned::Spanned;

#[derive(Debug, Default)]
pub struct TypeVisitor {
    pub errors: Vec<syn::Error>
}

impl<'ast> syn::visit::Visit<'ast> for TypeVisitor {
    fn visit_type(&mut self, ty: &'ast syn::Type) {
        if let syn::Type::Reference(ref reference) = ty {
            if reference.lifetime.is_none() {
                self.errors.push(
                    syn::Error::new(reference.span(), "In this function, all references must be annotated with a lifetime.")
                )
            }
            if let Some(syn::Lifetime {ident, ..}) = &reference.lifetime {
                if ident == "_" {
                    self.errors.push(
                        syn::Error::new(reference.lifetime.span(), "In this function, the anonymous lifetime is not allowed.")
                    )
                }
            }
        }

        syn::visit::visit_type(self, ty);
    }
}
