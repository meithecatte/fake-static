fn helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T { v }


fn helper_mut<'a, 'b, T>(_: &'a &'b (), v: &'b mut T) -> &'a mut T { v }

#[cfg(not(debug_assertions))]
compile_error!("Don't deploy this shit to production you madman");

/// Use black magic fuckery to turn any `&T` into a `&'static T`.
/// May introduce undefined behavior.
pub fn make_static<'a, T>(input: &'a T) -> &'static T {
    let f: fn(_, &'a T) -> &'static T = helper;
    f(&&(), input)
}

/// Use black magic fuckery to change the lifetime on a mutable refrence
/// May introduce undefined behavior.
pub fn make_static_mut<'a, 'b, T>(input: &'a mut T) -> &'b static T {
    let f: fn(_, &'a mut T) -> &'b static T = helper_mut;
    f(&&(), input)
}
