use std::sync::Arc;

use derive_new::new;

// Summaryトレイトを定義する
// Mock対象となるメソッドを定義する。
#[cfg_attr(test, mockall::automock)]
pub trait Summary {
    fn summarize(&self) -> String;
}

// Summary traitを実装した何かしらのsummaryを持つArticleを定義する
// 非同期を考慮したコードで同様のコードを前に書いたけど、テストダブルが渡せずコンパイルエラーになったのでその確認をする
#[allow(unused)]
#[derive(new)]
struct Article {
    summary: Arc<dyn Summary + Send + Sync>,
}


impl Article {
    #[allow(unused)]
    fn detail(&self) -> String {
        format!("summary: {}", self.summary.summarize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut mock_summary = MockSummary::new();
        mock_summary
            .expect_summarize()
            .returning(|| "mocked summary".to_string());
        let amock = Arc::new(mock_summary);
        // mockオブジェクトを含んだArticleを作成する
        // コンパイルで怒られない……なぜだ
        let article = Article::new(amock.clone());

        // テスト通るな
        assert_eq!(article.detail(), "summary: mocked summary")
    }
}
