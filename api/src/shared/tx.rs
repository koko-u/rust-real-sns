/// トランザクション中の操作であることを示すための &mut PgConnection のラッパーです
/// この型引数を関数に渡すことで、その関数がトランザクション内で実行されることを表現します
#[derive(Debug)]
pub struct Tx<'c>(sqlx::PgTransaction<'c>);

impl<'c> Tx<'c> {
    /// 非公開 コンストラクタ
    fn new(tx: sqlx::PgTransaction<'c>) -> Self {
        Self(tx)
    }

    /// コネクションオブジェクトへの可変参照を取り出します
    pub fn conn(&mut self) -> &mut sqlx::PgConnection {
        self.0.as_mut()
    }

    /// 非公開 commit
    async fn commit(self) -> Result<(), sqlx::Error> {
        self.0.commit().await
    }

    /// 非公開 rollback
    async fn rollback(self) -> Result<(), sqlx::Error> {
        self.0.rollback().await
    }
}

/// トランザクション内で、所定の処理 `f` (Result<T,E> を返却する クロージャ ) を実行します。
///
/// # examples
///
/// ```
/// let value = transaction(pool, async |tx| { ... }).await?
/// ```
pub async fn transaction<T, E, F>(pool: &sqlx::PgPool, f: F) -> Result<T, E>
where
    F: for<'a> AsyncFnOnce(&'a mut Tx<'_>) -> Result<T, E>,
    E: From<sqlx::Error>,
{
    let transaction = pool.begin().await?;

    let mut tx = Tx::new(transaction);

    let result = f(&mut tx).await;

    match result {
        Ok(value) => {
            tx.commit().await?;
            Ok(value)
        }
        Err(error) => {
            tx.rollback().await?;
            Err(error)
        }
    }
}
