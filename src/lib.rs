use std::{fs, path::Path};

use oxc::{
    allocator::Allocator,
    codegen::{Codegen, CodegenOptions},
    parser::Parser,
    span::SourceType,
    transformer::{TransformOptions, Transformer},
};

pub fn transform_path(path: &Path) -> String {
    let source_text = fs::read_to_string(path).unwrap();
    let source_type = SourceType::from_path(path).unwrap();
    transform(path, &source_text, source_type)
}

pub fn transform(path: &Path, source_text: &str, source_type: SourceType) -> String {
    let allocator = Allocator::default();

    let parser_ret = Parser::new(&allocator, source_text, source_type).parse();
    assert!(parser_ret.errors.is_empty(), "Expect no parse errors: {path:?}");

    let trivias = parser_ret.trivias;
    let mut program = parser_ret.program;

    let options = TransformOptions::default();
    Transformer::new(&allocator, path, source_type, source_text, &trivias, options)
        .build(&mut program)
        .unwrap_or_else(|_| panic!("Expect no transform errors: {path:?}"));

    let source_name = path.file_name().unwrap().to_string_lossy();
    let options = CodegenOptions::default();
    Codegen::<false>::new(&source_name, source_text, options, Default::default())
        .build(&program)
        .source_text
}
