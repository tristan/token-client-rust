#![feature(plugin_registrar, quote, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::codemap::Span;
use syntax::ast::Ident;
use syntax::tokenstream::TokenTree;
use syntax::symbol::Symbol;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager, IdentTT, get_single_str_from_tts};
use syntax::util::small_vector::SmallVector;
use rustc_plugin::Registry;

fn expand_mod_path_impl<'a>(cx: &'a mut ExtCtxt, sp: Span, is_pub: bool, ident: Ident, tts: Vec<TokenTree>)
                            -> Box<MacResult + 'a> {
    let path = match get_single_str_from_tts(cx, sp, &*tts, "mod_path!") {
        Some(string) => string,
        None => return DummyResult::expr(sp),
    };
    let path = &*path;

    if is_pub {
        MacEager::items(SmallVector::one(quote_item!(cx,
        #[path = $path]
        pub mod $ident;
        ).unwrap()))
    } else {
        MacEager::items(SmallVector::one(quote_item!(cx,
        #[path = $path]
        mod $ident;
        ).unwrap()))
    }
}
fn expand_mod_path<'a>(cx: &'a mut ExtCtxt, sp: Span, ident: Ident, tts: Vec<TokenTree>)
            -> Box<MacResult + 'a> {
    expand_mod_path_impl(cx, sp, false, ident, tts)
}

fn expand_pub_mod_path<'a>(cx: &'a mut ExtCtxt, sp: Span, ident: Ident, tts: Vec<TokenTree>)
                           -> Box<MacResult + 'a> {
    expand_mod_path_impl(cx, sp, true, ident, tts)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("mod_path"), IdentTT(Box::new(expand_mod_path), None, false));
    reg.register_syntax_extension(Symbol::intern("pub_mod_path"), IdentTT(Box::new(expand_pub_mod_path), None, false));
}
