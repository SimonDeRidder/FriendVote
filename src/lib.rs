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
pub mod app;
mod endpoints;

#[cfg(feature = "ssr")]
mod db;
#[cfg(feature = "ssr")]
mod entities;
#[cfg(feature = "ssr")]
mod ranked_pairs;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::app::*;
	console_error_panic_hook::set_once();
	leptos::mount::hydrate_body(App);
}
