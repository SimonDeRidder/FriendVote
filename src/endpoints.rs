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
use leptos::prelude::{server, ServerFnError};

#[server]
pub async fn create_election(election_name: String, candidates: Vec<String>) -> Result<(), ServerFnError> {
	let mut cleaned_candidates = candidates.clone();
	if cleaned_candidates.last().map(String::is_empty).unwrap_or(false) {
		cleaned_candidates.pop();
	}
	if cleaned_candidates.len() > 1 {
		// store the election
		use crate::db::insert_new_election;
		use leptos::prelude::use_context;
		use nanoid::nanoid;
		use sea_orm::DatabaseConnection;

		let db_conn = match use_context::<DatabaseConnection>() {
			Some(p) => p,
			None => {
				return Err(ServerFnError::ServerError(
					"Could not find db connection in context.".to_string(),
				));
			},
		};

		let election_id = nanoid!(16);
		let admin_id = nanoid!(16);
		let result_id = nanoid!(16);

		insert_new_election(
			&db_conn,
			&election_name,
			&cleaned_candidates,
			&election_id,
			&admin_id,
			&result_id,
		)
		.await?;
		println!("Successfully created election with name '{}'", election_name);

		// and redirect to the admin page
		leptos_axum::redirect(format!("{}/admin/{}", election_id, admin_id).as_str());
		Ok(())
	} else {
		Err(ServerFnError::ServerError("Not enough candidates.".to_string()))
	}
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AdminInfo {
	pub election_name: String,
	pub admin_path: String,
	pub vote_path: String,
	pub result_path: String,
}
#[server]
pub async fn get_election_admin_info(
	election_id: String,
	admin_id: String,
) -> Result<AdminInfo, ServerFnError> {
	use crate::db::get_election_details;
	use leptos::prelude::use_context;
	use sea_orm::DatabaseConnection;

	let db_conn = match use_context::<DatabaseConnection>() {
		Some(p) => p,
		None => {
			return Err(ServerFnError::ServerError("Could not find db connection in context.".to_string()));
		},
	};

	let db_entry = get_election_details(&db_conn, &election_id).await?;

	if admin_id == db_entry.admin_id {
		Ok(AdminInfo {
			election_name: db_entry.name,
			admin_path: format!("{}/admin/{}", db_entry.election_id, db_entry.admin_id),
			vote_path: format!("{}/vote", db_entry.election_id),
			result_path: format!("{}/result/{}", db_entry.election_id, db_entry.result_id),
		})
	} else {
		Err(ServerFnError::new("forbidden"))
	}
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VotingInfo {
	pub election_name: String,
	pub candidates: Vec<String>,
}
#[server]
pub async fn get_election_vote_info(election_id: String) -> Result<VotingInfo, ServerFnError> {
	use crate::db::get_election_details;
	use leptos::prelude::use_context;
	use sea_orm::DatabaseConnection;

	let db_conn = match use_context::<DatabaseConnection>() {
		Some(p) => p,
		None => {
			return Err(ServerFnError::ServerError("Could not find db connection in context.".to_string()));
		},
	};

	let db_entry = get_election_details(&db_conn, &election_id).await?;

	Ok(VotingInfo {
		election_name: db_entry.name,
		candidates: db_entry.candidates,
	})
}

#[server]
pub async fn cast_vote(
	election_id: String,
	candidates: Vec<String>,
	comp: Vec<String>,
) -> Result<(), ServerFnError> {
	use crate::db::{get_election_details, insert_vote};
	use leptos::prelude::use_context;
	use leptos::server_fn::error::NoCustomError;
	use sea_orm::DatabaseConnection;

	// calculate comparison states from checkbox ids
	let mut comparator_is_bigger = Vec::with_capacity(candidates.len() - 1);
	let mut comp_ind;
	let mut curr_ind = 0;
	for comp_name in comp {
		comp_ind = comp_name
			.split("_")
			.last()
			.ok_or(ServerFnError::<NoCustomError>::ServerError(
				"Could not find index in comp name.".to_string(),
			))?
			.parse::<usize>()?;
		if comp_ind >= (candidates.len() - 1) {
			break;
		}
		for _ind in curr_ind..comp_ind {
			comparator_is_bigger.push(false);
		}
		comparator_is_bigger.push(true);
		curr_ind = comp_ind + 1;
	}
	for _ind in curr_ind..(candidates.len() - 1) {
		comparator_is_bigger.push(false);
	}

	// get election info
	let db_conn = match use_context::<DatabaseConnection>() {
		Some(p) => p,
		None => {
			return Err(ServerFnError::ServerError("Could not find db connection in context.".to_string()));
		},
	};
	let db_entry = get_election_details(&db_conn, &election_id).await?;

	// get sorted indices
	if candidates.len() != db_entry.candidates.len() {
		return Err(ServerFnError::ServerError(format!(
			"Number of candidates does not match: {}",
			candidates.len()
		)));
	}
	let mut candidate_order = Vec::with_capacity(candidates.len());
	let mut found;
	for candidate in candidates {
		found = false;
		for (ind, orig_candidate) in db_entry.candidates.iter().enumerate() {
			if candidate == *orig_candidate {
				candidate_order.push(ind as i32);
				found = true;
				break;
			}
		}
		if !found {
			return Err(ServerFnError::ServerError(format!("Illegal candidate: {candidate}")));
		}
	}

	// insert the vote
	insert_vote(&db_conn, &election_id, &candidate_order, &comparator_is_bigger).await?;

	// and redirect to the vote thanks page
	leptos_axum::redirect("vote_thanks");
	Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ElectionResults {
	pub election_name: String,
	pub candidates: Vec<String>,
	pub n_votes: u64,
	pub defeats_matrix: Vec<Vec<Option<u64>>>,
	pub ranked_candidates: Vec<Vec<String>>,
}
#[server]
pub async fn get_election_results(
	election_id: String,
	result_id: String,
) -> Result<ElectionResults, ServerFnError> {
	use crate::db::get_election_details;
	use crate::entities::prelude::Votes;
	use crate::entities::votes;
	use crate::ranked_pairs::calculate_ranks;
	use leptos::prelude::use_context;
	use sea_orm::{ColumnTrait as _, DatabaseConnection, EntityTrait as _, QueryFilter as _};

	let db_conn = match use_context::<DatabaseConnection>() {
		Some(p) => p,
		None => {
			return Err(ServerFnError::ServerError("Could not find db connection in context.".to_string()));
		},
	};

	let db_entry = get_election_details(&db_conn, &election_id).await?;

	if result_id != db_entry.result_id {
		return Err(ServerFnError::new("forbidden"));
	}

	// fetch votes
	let votes = Votes::find()
		.filter(votes::Column::ElectionId.eq(election_id))
		.all(&db_conn)
		.await?;

	// calculate defeats matrix
	let mut defeats_matrix = Vec::with_capacity(db_entry.candidates.len());
	{
		for i in 0..db_entry.candidates.len() {
			defeats_matrix.push(vec![Some(0u64); db_entry.candidates.len()]);
			defeats_matrix[i][i] = None;
		}
		let mut other_ind;
		let mut bigger;
		for vote in votes.iter() {
			for (i, ind) in vote.cand_order.iter().enumerate() {
				bigger = false;
				for j in (i + 1)..vote.cand_order.len() {
					other_ind = vote.cand_order[j];
					bigger = bigger || vote.comp_is_bigger[j - 1];
					if bigger {
						defeats_matrix[*ind as usize][other_ind as usize] =
							defeats_matrix[*ind as usize][other_ind as usize].map(|x| x + 1u64);
					}
				}
			}
		}
	}

	// calculate ranks
	let mut ranked_candidates = Vec::new();
	{
		let cand_ranks = calculate_ranks(&defeats_matrix);
		let max_rank = *cand_ranks.values().max().unwrap();
		for _ in 0..(max_rank + 1) {
			ranked_candidates.push(Vec::new());
		}
		for (cand_ind, rank) in cand_ranks.iter() {
			ranked_candidates[*rank].push(db_entry.candidates[*cand_ind].clone());
		}
	}

	Ok(ElectionResults {
		election_name: db_entry.name,
		candidates: db_entry.candidates,
		n_votes: votes.len() as u64,
		defeats_matrix: defeats_matrix,
		ranked_candidates: ranked_candidates,
	})
}
