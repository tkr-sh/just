//! `just` is primarily used as a command-line binary, but does provide a
//! limited public library interface.
//!
//! Please keep in mind that there are no semantic version guarantees for the
//! library interface. It may break or change at any time.

// Used in integration tests.
#[doc(hidden)]
pub use unindent::unindent;
pub use {
    crate::{
        alias::Alias,
        analyzer::Analyzer,
        argument_parser::ArgumentParser,
        assignment::Assignment,
        assignment_resolver::AssignmentResolver,
        ast::Ast,
        attribute::Attribute,
        binding::Binding,
        color::Color,
        color_display::ColorDisplay,
        command_color::CommandColor,
        command_ext::CommandExt,
        compilation::Compilation,
        compile_error::CompileError,
        compile_error_kind::CompileErrorKind,
        compiler::Compiler,
        condition::Condition,
        conditional_operator::ConditionalOperator,
        config::Config,
        config_error::ConfigError,
        constants::constants,
        count::Count,
        delimiter::Delimiter,
        dependency::Dependency,
        dump_format::DumpFormat,
        enclosure::Enclosure,
        error::Error,
        evaluator::Evaluator,
        execution_context::ExecutionContext,
        executor::Executor,
        expression::Expression,
        fragment::Fragment,
        function::Function,
        interpreter::Interpreter,
        interrupt_guard::InterruptGuard,
        interrupt_handler::InterruptHandler,
        item::Item,
        justfile::Justfile,
        keyed::Keyed,
        keyword::Keyword,
        lexer::Lexer,
        line::Line,
        list::List,
        load_dotenv::load_dotenv,
        loader::Loader,
        module_path::ModulePath,
        name::Name,
        namepath::Namepath,
        ordinal::Ordinal,
        output::output,
        output_error::OutputError,
        parameter::Parameter,
        parameter_kind::ParameterKind,
        parser::Parser,
        platform::Platform,
        platform_interface::PlatformInterface,
        position::Position,
        positional::Positional,
        ran::Ran,
        range_ext::RangeExt,
        recipe::Recipe,
        recipe_resolver::RecipeResolver,
        recipe_signature::RecipeSignature,
        run::run,
        scope::Scope,
        search::Search,
        search_config::SearchConfig,
        search_error::SearchError,
        set::Set,
        setting::Setting,
        settings::Settings,
        shebang::Shebang,
        show_whitespace::ShowWhitespace,
        source::Source,
        string_delimiter::StringDelimiter,
        string_kind::StringKind,
        string_literal::StringLiteral,
        subcommand::Subcommand,
        suggestion::Suggestion,
        table::Table,
        thunk::Thunk,
        token::Token,
        token_kind::TokenKind,
        unresolved_dependency::UnresolvedDependency,
        unresolved_recipe::UnresolvedRecipe,
        unstable_feature::UnstableFeature,
        use_color::UseColor,
        variables::Variables,
        verbosity::Verbosity,
        warning::Warning,
    },
    camino::Utf8Path,
    clap::ValueEnum,
    derive_where::derive_where,
    edit_distance::edit_distance,
    lexiclean::Lexiclean,
    libc::EXIT_FAILURE,
    once_cell::sync::Lazy,
    regex::Regex,
    serde::{
        ser::{SerializeMap, SerializeSeq},
        Serialize,
        Serializer,
    },
    snafu::{ResultExt, Snafu},
    std::{
        borrow::Cow,
        cmp,
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        env,
        ffi::OsString,
        fmt::{self, Debug, Display, Formatter},
        fs,
        io::{self, Read, Seek, Write},
        iter::{self, FromIterator},
        mem,
        ops::{Deref, Index, Range, RangeInclusive},
        path::{self, Path, PathBuf},
        process::{self, Command, ExitStatus, Stdio},
        rc::Rc,
        str::{self, Chars},
        sync::{Mutex, MutexGuard, OnceLock},
        vec,
    },
    strum::{Display, EnumDiscriminants, EnumString, IntoStaticStr},
    tempfile::tempfile,
    typed_arena::Arena,
    unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
};

#[cfg(test)]
pub use crate::{node::Node, tree::Tree};

type CompileResult<'a, T = ()> = Result<T, CompileError<'a>>;
type ConfigResult<T> = Result<T, ConfigError>;
type FunctionResult = Result<String, String>;
pub type RunResult<'a, T = ()> = Result<T, Error<'a>>;
type SearchResult<T> = Result<T, SearchError>;

#[cfg(test)]
#[macro_use]
pub mod testing;

#[cfg(test)]
#[macro_use]
pub mod tree;

#[cfg(test)]
pub mod node;

#[cfg(fuzzing)]
pub mod fuzzing;

// Used by Janus, https://github.com/casey/janus, a tool
// that analyses all public justfiles on GitHub to avoid
// breaking changes.
#[doc(hidden)]
pub mod summary;

pub mod alias;
pub mod analyzer;
pub mod argument_parser;
pub mod assignment;
pub mod assignment_resolver;
pub mod ast;
pub mod attribute;
pub mod binding;
pub mod color;
pub mod color_display;
pub mod command_color;
pub mod command_ext;
pub mod compilation;
pub mod compile_error;
pub mod compile_error_kind;
pub mod compiler;
pub mod completions;
pub mod condition;
pub mod conditional_operator;
pub mod config;
pub mod config_error;
pub mod constants;
pub mod count;
pub mod delimiter;
pub mod dependency;
pub mod dump_format;
pub mod enclosure;
pub mod error;
pub mod evaluator;
pub mod execution_context;
pub mod executor;
pub mod expression;
pub mod fragment;
pub mod function;
pub mod interpreter;
pub mod interrupt_guard;
pub mod interrupt_handler;
pub mod item;
pub mod justfile;
pub mod keyed;
pub mod keyword;
pub mod lexer;
pub mod line;
pub mod list;
pub mod load_dotenv;
pub mod loader;
pub mod module_path;
pub mod name;
pub mod namepath;
pub mod ordinal;
pub mod output;
pub mod output_error;
pub mod parameter;
pub mod parameter_kind;
pub mod parser;
pub mod platform;
pub mod platform_interface;
pub mod position;
pub mod positional;
pub mod ran;
pub mod range_ext;
pub mod recipe;
pub mod recipe_resolver;
pub mod recipe_signature;
pub mod run;
pub mod scope;
pub mod search;
pub mod search_config;
pub mod search_error;
pub mod set;
pub mod setting;
pub mod settings;
pub mod shebang;
pub mod show_whitespace;
pub mod source;
pub mod string_delimiter;
pub mod string_kind;
pub mod string_literal;
pub mod subcommand;
pub mod suggestion;
pub mod table;
pub mod thunk;
pub mod token;
pub mod token_kind;
pub mod unindent;
pub mod unresolved_dependency;
pub mod unresolved_recipe;
pub mod unstable_feature;
pub mod use_color;
pub mod variables;
pub mod verbosity;
pub mod warning;
