//! Корневой Query GraphQL-схемы. Каждый метод `impl Query` — поле графа (резолвер).
//!
//! Резолверы могут получать пул БД из контекста: `ctx.data::<sqlx::PgPool>()?`,
//! и со временем — вызывать те же usecase доменов, что и REST-хендлеры.

use async_graphql::Object;

/// Корень GraphQL-запросов.
pub struct Query;

#[Object]
impl Query {
    /// Версия API.
    async fn api_version(&self) -> &'static str {
        "0.1.0"
    }
}
