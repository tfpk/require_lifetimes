mod type_visitor;
use type_visitor::TypeVisitor;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::Item;

/// This attribute allows you to require that a function have
/// all lifetimes in its input and output be present.
/// A missing lifetime will cause a compiler error.
///
/// ## Examples
///
///
/// This code compiles, as all required lifetimes are present:
///
/// ```rust
/// use require_lifetimes::require_lifetimes;
///
/// #[require_lifetimes]
/// fn identity<'a>(a: &'a i32) -> &'a i32 {
///     a
/// }
/// ```
///
/// This code, however doesn't. Note that if the lifetimes
/// aren't there, by default this macro outputs the exact same code.
/// This means that the user may get further errors based on that
/// code. To force the macro to output nothing, provide `!` as
/// an argument to the macro invocation.
///
/// ```rust,compile_fail
/// use require_lifetimes::require_lifetimes;
/// // This fails to compile, even though lifetime elision
/// // means it should be fine.
///
/// #[require_lifetimes(!)]
/// fn identity(a: &i32) -> &i32 {
///     a
/// }
/// ```
///
/// In some circumstances, you may want to replace the item
/// that's missing lifetimes. To do that, you can put code
/// in brackets after the macro.
///
/// ```rust,compile_fail
/// use require_lifetimes::require_lifetimes;
/// // This fails to compile, and will print an error for
/// // both the missing lifetimes, and for the `compile_error!`
/// // macro.
///
/// #[require_lifetimes(compile_error!("identity function was removed."))]
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
    let attr_span = attr.span();
    let ast: Item = syn::parse(item.clone()).unwrap();
    return match ast {
        Item::Fn(ref func) => {
            let mut visitor = TypeVisitor::default();
            visitor.visit_signature(&func.sig);

            let lifetime_errors = visitor.errors;
            if lifetime_errors.is_empty() {
                return item;
            }
            let function = func.clone();

            let new_function_stream = parse_error_case_from_attribute(attr);
            let mut new_function_stream = proc_macro::TokenStream::from(new_function_stream.unwrap_or_else(|| function.to_token_stream()));

            let error_stream = proc_macro::TokenStream::from_iter(
                lifetime_errors
                    .iter()
                    .map(|e| Into::<proc_macro::TokenStream>::into(e.to_compile_error())),
            );

            new_function_stream.extend(error_stream);
            new_function_stream
        }
        _ => {
            error_to_final_token_stream(item, &attr_span,  "Attribute not valid on this item type.")
        }
    };
}

/// Given a `span` which contains an error, and an error message `message`,
/// return a `proc_macro::TokenStream` which encapsulates that error.
fn error_to_token_stream(span: &proc_macro2::Span, message: &'static str) -> proc_macro::TokenStream {
    let error = syn::Error::new(*span, message).to_compile_error();
    proc_macro::TokenStream::from(error)
}

/// Given an error message, a span, and some `TokenStream` representing
/// the new code outputted by the macro; append the error to the new code.
fn error_to_final_token_stream(mut body: proc_macro::TokenStream, span: &proc_macro2::Span, message: &'static str) -> proc_macro::TokenStream {
    let error = error_to_token_stream(span, message);
    body.extend(error);
    body
}

/// Given an attribute for `require_lifetimes`, get the body of
/// the attribute (or an error). This allows the user to replace
/// broken code with nothing (or another default block of code).
fn parse_error_case_from_attribute(attr: proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream> {
    if attr.is_empty() {
        None
    } else if attr.to_string() == "!" {
        Some(proc_macro2::TokenStream::new())
    } else {
        Some(attr)
    }
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
