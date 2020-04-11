use {
    eventual::Async,
    unicode::UString,
    jqsh::{
        builtin,
        lang::{
            Filter,
            channel,
            parser
        }
    }
};

fn main() {
    let mut repl_context = builtin::context();
    while let Some(source_utf8) = readline::readline("jqsh> ") {
        readline::add_history(&source_utf8);
        let source = UString::from(source_utf8);
        let filter = parser::parse(source, repl_context.clone()).unwrap_or_else(|err| {
            println!("jqsh: syntax error: {:?}", err);
            Filter::Empty
        });
        let channel::Receiver { context, values } = channel::Receiver::empty(repl_context).filter(&filter);
        repl_context = context.r#await().expect("failed to get repl output context");
        for value in values {
            println!("{}", value);
        }
    }
    println!("");
}
