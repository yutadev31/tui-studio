pub const C: &str = "c";
pub const C_HEADER: &str = "c-header";
pub const CPP: &str = "cpp";
pub const CPP_HEADER: &str = "cpp-header";
pub const HTML: &str = "html";
pub const CSS: &str = "css";
pub const JAVASCRIPT: &str = "javascript";
pub const TYPESCRIPT: &str = "typescript";
pub const JAVASCRIPT_REACT: &str = "javascript-react";
pub const TYPESCRIPT_REACT: &str = "typescript-react";
pub const PHP: &str = "php";
pub const RUBY: &str = "ruby";
pub const PYTHON: &str = "python";
pub const RUST: &str = "rust";
pub const GO: &str = "go";
pub const JAVA: &str = "java";
pub const KOTLIN: &str = "kotlin";
pub const SWIFT: &str = "swift";
pub const SHELL: &str = "shell";
pub const POWERSHELL: &str = "powershell";
pub const PERL: &str = "perl";
pub const LUA: &str = "lua";
pub const JSON: &str = "json";
pub const YAML: &str = "yaml";
pub const XML: &str = "xml";
pub const CSV: &str = "csv";
pub const SQL: &str = "sql";
pub const MARKDOWN: &str = "markdown";
pub const TEX: &str = "tex";
pub const MAKEFILE: &str = "makefile";
pub const DOCKERFILE: &str = "dockerfile";
pub const ASSEMBLY: &str = "assembly";
pub const HASKELL: &str = "haskell";
pub const ERLANG: &str = "erlang";
pub const ELIXIR: &str = "elixir";
pub const OCAML: &str = "ocaml";
pub const FSHARP: &str = "fsharp";
pub const SCALA: &str = "scala";
pub const CLOJURE: &str = "clojure";
pub const GROOVY: &str = "groovy";
pub const FORTRAN: &str = "fortran";
pub const DART: &str = "dart";
pub const OBJECTIVE_C: &str = "objective-c";
pub const CSHARP: &str = "csharp";
pub const COBOL: &str = "cobol";
pub const PLANETEXT: &str = "planetext";
pub const TOML: &str = "toml";
pub const GIT_IGNORE: &str = "git-ignore";
pub const COMMIT_MESSAGE: &str = "commit-message";

pub struct FileType {
    name: String,
}

impl FileType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn get(&self) -> String {
        self.name.clone()
    }

    pub fn file_name_to_type(file_name: String) -> Self {
        let extension = file_name.split('.').last();

        match file_name.as_str() {
            "Makefile" => return Self::new(MAKEFILE),
            "Dockerfile" => return Self::new(DOCKERFILE),
            ".gitignore" => return Self::new(GIT_IGNORE),
            "COMMIT_EDITMSG" => return Self::new(COMMIT_MESSAGE),
            _ => {}
        }

        if let Some(extension) = extension {
            match extension {
                "c" => return Self::new(C),
                "h" => return Self::new(C_HEADER),
                "cpp" => return Self::new(CPP),
                "cxx" => return Self::new(CPP),
                "cc" => return Self::new(CPP),
                "hpp" => return Self::new(CPP_HEADER),
                "hxx" => return Self::new(CPP_HEADER),
                "hh" => return Self::new(CPP_HEADER),
                "html" => return Self::new(HTML),
                "htm" => return Self::new(HTML),
                "css" => return Self::new(CSS),
                "js" => return Self::new(JAVASCRIPT),
                "ts" => return Self::new(TYPESCRIPT),
                "jsx" => return Self::new(JAVASCRIPT_REACT),
                "tsx" => return Self::new(TYPESCRIPT_REACT),
                "php" => return Self::new(PHP),
                "rb" => return Self::new(RUBY),
                "py" => return Self::new(PYTHON),
                "rs" => return Self::new(RUST),
                "go" => return Self::new(GO),
                "java" => return Self::new(JAVA),
                "kt" => return Self::new(KOTLIN),
                "kts" => return Self::new(KOTLIN),
                "swift" => return Self::new(SWIFT),
                "sh" => return Self::new(SHELL),
                "bash" => return Self::new(SHELL),
                "zsh" => return Self::new(SHELL),
                "ps1" => return Self::new(POWERSHELL),
                "pl" => return Self::new(PERL),
                "pm" => return Self::new(PERL),
                "lua" => return Self::new(LUA),
                "json" => return Self::new(JSON),
                "yml" => return Self::new(YAML),
                "yaml" => return Self::new(YAML),
                "xml" => return Self::new(XML),
                "csv" => return Self::new(CSV),
                "sql" => return Self::new(SQL),
                "md" => return Self::new(MARKDOWN),
                "markdown" => return Self::new(MARKDOWN),
                "tex" => return Self::new(TEX),
                "asm" => return Self::new(ASSEMBLY),
                "s" => return Self::new(ASSEMBLY),
                "hs" => return Self::new(HASKELL),
                "lhs" => return Self::new(HASKELL),
                "erl" => return Self::new(ERLANG),
                "hrl" => return Self::new(ERLANG),
                "ex" => return Self::new(ELIXIR),
                "exs" => return Self::new(ELIXIR),
                "ml" => return Self::new(OCAML),
                "mli" => return Self::new(OCAML),
                "fs" => return Self::new(FSHARP),
                "fsi" => return Self::new(FSHARP),
                "fsx" => return Self::new(FSHARP),
                "scala" => return Self::new(SCALA),
                "sc" => return Self::new(SCALA),
                "clj" => return Self::new(CLOJURE),
                "cljs" => return Self::new(CLOJURE),
                "cljc" => return Self::new(CLOJURE),
                "edn" => return Self::new(CLOJURE),
                "groovy" => return Self::new(GROOVY),
                "gvy" => return Self::new(GROOVY),
                "gy" => return Self::new(GROOVY),
                "gsh" => return Self::new(GROOVY),
                "f" => return Self::new(FORTRAN),
                "for" => return Self::new(FORTRAN),
                "f90" => return Self::new(FORTRAN),
                "f95" => return Self::new(FORTRAN),
                "dart" => return Self::new(DART),
                "m" => return Self::new(OBJECTIVE_C),
                "mm" => return Self::new(OBJECTIVE_C),
                "cs" => return Self::new(CSHARP),
                "cbl" => return Self::new(COBOL),
                "cob" => return Self::new(COBOL),
                "toml" => return Self::new(TOML),
                _ => {}
            }
        }

        Self::new(PLANETEXT)
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::new(PLANETEXT)
    }
}
