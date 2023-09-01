use std::path::PathBuf;

use deno_ast::parse_module;
use deno_ast::swc::ast::Decl;
use deno_ast::swc::ast::FnDecl;
use deno_ast::swc::ast::Stmt;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Empty out the wrappers dir
    for entry in std::fs::read_dir("./wrappers")? {
        std::fs::remove_file(entry?.path())?;
    }

    // read the tests dir
    for entry in std::fs::read_dir("./tests")? {
        let file = entry?.path();
        if file.to_str().unwrap().ends_with(".ts") {
            println!("Name: {}", file.display());
            do_file(file)?;
        }
    }

    Ok(())
}

fn do_file(path: PathBuf) -> std::io::Result<()> {
    let mut output_file = std::fs::File::create(format!(
        "./wrappers/{}",
        path.as_path().file_name().unwrap().to_str().unwrap()
    ))?;

    let specifier = path.to_str().unwrap().to_owned();
    let source_text = std::fs::read_to_string(path).unwrap();
    let parsed_source = parse_module(ParseParams {
        specifier,
        media_type: MediaType::TypeScript,
        text_info: SourceTextInfo::new(source_text.into()),
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    })
    .expect("should parse");

    let module = parsed_source.module();

    println!("{:?}", module);

    for item in module.body.iter() {
        match item {
            deno_ast::swc::ast::ModuleItem::ModuleDecl(_) => {}
            deno_ast::swc::ast::ModuleItem::Stmt(s) => match s {
                Stmt::Block(_) => {}
                Stmt::Empty(_) => {}
                Stmt::Debugger(_) => {}
                Stmt::With(_) => {}
                Stmt::Return(_) => {}
                Stmt::Labeled(_) => {}
                Stmt::Break(_) => {}
                Stmt::Continue(_) => {}
                Stmt::If(_) => {}
                Stmt::Switch(_) => {}
                Stmt::Throw(_) => {}
                Stmt::Try(_) => {}
                Stmt::While(_) => {}
                Stmt::DoWhile(_) => {}
                Stmt::For(_) => {}
                Stmt::ForIn(_) => {}
                Stmt::ForOf(_) => {}
                Stmt::Decl(d) => match d {
                    Decl::Class(_) => {}
                    Decl::Fn(FnDecl {
                        ident,
                        declare,
                        function,
                    }) => {
                        let sym = ident.sym.to_string();
                        println!("{sym:?}");
                        write!(
                            output_file,
                            "function {sym}() {{ playwright_something.exec(\"{sym}()\") }}\n\n"
                        )?;
                    }
                    Decl::Var(_) => {}
                    Decl::Using(_) => {}
                    Decl::TsInterface(_) => {}
                    Decl::TsTypeAlias(_) => {}
                    Decl::TsEnum(_) => {}
                    Decl::TsModule(_) => {}
                },
                Stmt::Expr(_) => {}
            },
        };
    }

    Ok(())
}
