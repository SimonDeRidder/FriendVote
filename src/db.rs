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
use crate::entities::election;
use crate::entities::prelude::Election;
use crate::entities::votes;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait as _};

#[derive(Debug)]
pub struct DbError {
	message: String,
}

impl std::fmt::Display for DbError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "DbError({})", self.message)
	}
}

impl std::error::Error for DbError {}

impl From<sea_orm::DbErr> for DbError {
	fn from(value: sea_orm::DbErr) -> Self {
		Self {
			message: value.to_string(),
		}
	}
}

pub async fn insert_new_election(
	db_conn: &DatabaseConnection,
	name: &String,
	candidates: &Vec<String>,
	election_id: &String,
	admin_id: &String,
	result_id: &String,
) -> Result<(), DbError> {
	election::ActiveModel {
		election_id: ActiveValue::Set(election_id.clone()),
		name: ActiveValue::Set(name.clone()),
		candidates: ActiveValue::Set(candidates.clone()),
		admin_id: ActiveValue::Set(admin_id.clone()),
		result_id: ActiveValue::Set(result_id.clone()),
	}
	.insert(db_conn)
	.await?;
	Ok(())
}

pub struct ElectionInfo {
	pub election_id: String,
	pub admin_id: String,
	pub result_id: String,
	pub name: String,
	pub candidates: Vec<String>,
}

pub async fn get_election_details(
	db_conn: &DatabaseConnection,
	election_id: &String,
) -> Result<ElectionInfo, DbError> {
	let db_row = Election::find_by_id(election_id)
		.one(db_conn)
		.await?
		.ok_or(DbError {
			message: format!("Could not find election with id {}", election_id),
		})?;
	Ok(ElectionInfo {
		election_id: db_row.election_id,
		admin_id: db_row.admin_id,
		result_id: db_row.result_id,
		name: db_row.name,
		candidates: db_row.candidates,
	})
}

pub async fn insert_vote(
	db_conn: &DatabaseConnection,
	election_id: &String,
	candidate_order: &Vec<i32>,
	comparison_is_bigger: &Vec<bool>,
) -> Result<(), DbError> {
	votes::ActiveModel {
		vote_id: ActiveValue::NotSet,
		election_id: ActiveValue::Set(election_id.clone()),
		cand_order: ActiveValue::Set(candidate_order.clone()),
		comp_is_bigger: ActiveValue::Set(comparison_is_bigger.clone()),
	}
	.insert(db_conn)
	.await?;
	Ok(())
}
