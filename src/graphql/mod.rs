//! GraphQL-слой: схема, эндпоинт `/graphql` и песочница GraphiQL.
//!
//! Это альтернативный «вход» к той же логике, что и REST: резолверы (в `query.rs`)
//! со временем вызывают те же usecase доменов. Пул БД прокинут в схему через
//! `.data()` и доступен резолверам как `ctx.data::<PgPool>()`.

use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Router;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use sqlx::PgPool;

use crate::shared::state::AppState;

mod query;
use query::Query;

/// Тип скомпилированной GraphQL-схемы приложения.
pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Строит GraphQL-схему. Пул БД доступен резолверам через `ctx.data::<PgPool>()`.
pub fn build_schema(db: PgPool) -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}

/// Песочница GraphiQL (GET `/graphql` в браузере).
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Выполняет GraphQL-запрос (POST `/graphql`).
async fn graphql_handler(State(state): State<AppState>, req: GraphQLRequest) -> GraphQLResponse {
    state.graphql.execute(req.into_inner()).await.into()
}

/// Роуты GraphQL: GET — песочница, POST — запросы (оба на `/graphql`).
pub fn router() -> Router<AppState> {
    Router::new().route("/graphql", get(graphiql).post(graphql_handler))
}
