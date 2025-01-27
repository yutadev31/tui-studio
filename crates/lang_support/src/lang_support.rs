pub mod complete;
pub mod highlight;
pub mod lint;
pub mod snippets;

use std::any::Any;

use complete::CompletionItem;
use highlight::SyntaxDefinition;
use lint::LintError;
use snippets::Snippet;

// ランゲージサポート
pub trait LanguageSupport: Any + Send + Sync {
    fn file_type(&self) -> &'static str;

    // シンタックスハイライト用の構文を取得
    fn get_syntax_definition(&self) -> Option<&SyntaxDefinition> {
        None
    }

    /// 補完候補を取得
    fn complete(&self, cursor_position: usize, source_code: &str) -> Option<Vec<CompletionItem>> {
        let _ = (cursor_position, source_code);
        None
    }

    /// エラーチェックを実行
    fn lint(&self, source_code: &str) -> Option<Vec<LintError>> {
        let _ = source_code;
        None
    }

    /// フォーマット済みコードを返す
    fn format(&self, source_code: &str) -> Option<String> {
        let _ = source_code;
        None
    }

    /// スニペットのリストを取得
    fn snippets(&self) -> Option<Vec<Snippet>> {
        None
    }
}
