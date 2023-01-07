mod type_visitor;
use type_visitor::TypeVisitor;

use syn::spanned::Spanned;
use syn::Item;
use syn::visit::Visit;


/// This attribute allows you to require that a function have
/// all lifetimes in its input and output be present.
/// A missing lifetime will cause a compiler error.
///
/// ## Examples
///
/// ```rust
/// use require_lifetimes::require_lifetimes;
/// // This compiles fine...
///
/// #[require_lifetimes]
/// fn identity<'a>(a: &'a i32) -> &'a i32 {
///     a
/// }
/// ```
///
/// ```rust,compile_fail
/// use require_lifetimes::require_lifetimes;
/// // This fails to compile, even though lifetime elision
/// // means it should be fine.
///
/// #[require_lifetimes]
/// fn identity(a: &i32) -> &i32 {
///     a
/// }
/// ```
#[proc_macro_attribute]
pub fn require_lifetimes(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let ast: Item = syn::parse(item.clone()).unwrap();
    match ast {
        Item::Fn(ref func) => {
            let mut visitor = TypeVisitor::default();
            visitor.visit_signature(&func.sig);
            let lifetime_errors = visitor.errors;
            if !lifetime_errors.is_empty() {
                return proc_macro::TokenStream::from_iter(
                    lifetime_errors
                        .iter()
                        .map(|e| Into::<proc_macro::TokenStream>::into(e.to_compile_error())),
                );
            }
        }
        _ => {
            let error = syn::Error::new(attr.span(), "Attribute not valid on this item type.")
                .to_compile_error();
            return proc_macro::TokenStream::from(error);
        }
    }
    item
}

#[cfg(test)]
mod test {
    #[test]
    fn ui_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("ui_tests/*_err.rs");
        t.pass("ui_tests/*_ok.rs");
    }
}
