/// Copyright 2025 Simon De Ridder
/// This file is part of FriendVote.
/// FriendVote is free software: you can redistribute it and/or modify it under the terms of the
/// GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
/// or (at your option) any later version.
/// FriendVote is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
/// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
/// See the GNU General Public License for more details.
/// You should have received a copy of the GNU General Public License along with FriendVote.
/// If not, see <https://www.gnu.org/licenses/>.
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use std::time::Duration;

	use axum::Router;
	use friendvote::app::*;
	use leptos::logging::log;
	use leptos::prelude::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};
	use sea_orm::{ConnectOptions, Database};

	use migration::{Migrator, MigratorTrait as _};

	let conf = get_configuration(None).unwrap();
	let addr = conf.leptos_options.site_addr;
	let leptos_options = conf.leptos_options;
	// Generate the list of routes in your Leptos App
	let routes = generate_route_list(App);

	// Set up db connection
	// postgres://<user>:<passwd>@localhost/friendvote?currentSchema=public
	let db_connection_str = std::env::var("DATABASE_URL").expect("env variable DATABASE_URL not set");
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::DEBUG)
		.with_test_writer()
		.init();
	let mut db_options = ConnectOptions::new(db_connection_str.clone());
	db_options
		.max_connections(5)
		.acquire_timeout(Duration::from_secs(3))
		.sqlx_logging(false); // Disable SQLx log
	let db_conn = Database::connect(db_options)
		.await
		.expect("unable to connect to database");
	Migrator::up(&db_conn, None)
		.await
		.expect("migration did not succeed");

	let app = Router::new()
		.leptos_routes_with_context(&leptos_options, routes, move || provide_context(db_conn.clone()), {
			let leptos_options = leptos_options.clone();
			move || shell(leptos_options.clone())
		})
		.fallback(leptos_axum::file_and_error_handler(shell))
		.with_state(leptos_options);

	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	log!("listening on http://{}", &addr);
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for pure client-side testing
	// see lib.rs for hydration function instead
}
