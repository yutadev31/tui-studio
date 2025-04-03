use std::any::Any;

use super::highlight::HighlightToken;

pub trait LanguageSupport: Any + Send + Sync {
    // シンタックスハイライト
    fn highlight(&self, source_code: &str) -> Option<Vec<HighlightToken>> {
        let _ = source_code;
        None
    }

    // /// 補完候補を取得
    // fn complete(&self, cursor_position: usize, source_code: &str) -> Option<Vec<CompletionItem>> {
    //     let _ = (cursor_position, source_code);
    //     None
    // }
    //
    // /// エラーチェックを実行
    // fn lint(&self, source_code: &str) -> Option<Vec<LintError>> {
    //     let _ = source_code;
    //     None
    // }
    //
    // /// フォーマット済みコードを返す
    // fn format(&self, source_code: &str) -> Option<String> {
    //     let _ = source_code;
    //     None
    // }
    //
    // /// スニペットのリストを取得
    // fn snippets(&self) -> Option<Vec<Snippet>> {
    //     None
    // }
    //
    // /// LSPサーバーのコマンドを取得
    // fn get_lsp_server_cmd(&self) -> Option<String> {
    //     None
    // }
}
