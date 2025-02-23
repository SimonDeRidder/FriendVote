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
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
	components::{Outlet, ParentRoute, Route, Router, Routes},
	hooks::use_params,
	params::Params,
	path,
};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

// use crate::components::{drag_list::DragList, ListItem};
use crate::endpoints;

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<AutoReload options=options.clone() />
				<HydrationScripts options/>
				<MetaTags/>
			</head>
			<body>
				<App/>
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/friendvote.css"/>

		// sets the document title
		<Title text="FriendVote"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes fallback=|| "Page not found.".into_view()>
					<Route path=path!("") view=HomePage/>
					<Route path=path!("vote_thanks") view=VoteThanksPage/>
					<ParentRoute path=path!(":election_id") view=VoteParent>
						<Route path=path!("admin/:admin_id") view=AdminPage/>
						<Route path=path!("vote") view=VotePage/>
						<Route path=path!("result/:result_id") view=ResultPage/>
					</ParentRoute>
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage() -> impl IntoView {
	let create_election = ServerAction::<endpoints::CreateElection>::new();
	let candidates = RwSignal::new(Vec::<(RwSignal<bool>, RwSignal<String>)>::new());
	candidates
		.write_untracked()
		.push((RwSignal::new(true), RwSignal::new(String::new())));
	candidates
		.write_untracked()
		.push((RwSignal::new(true), RwSignal::new(String::new())));

	view! {
		<h1>"Welcome to FriendVote!"</h1>
		<div style="display:flex;justify-content:center">
			<ActionForm action=create_election>
				<div style="margin:5px;margin-bottom:10px">
					<label for="election_name">"Election name:"</label>
					<input id="election_name" type="text" name="election_name"/>
				</div>
				<For
					each=move || {candidates.get().into_iter().enumerate().collect::<Vec<_>>()}
					key=move |(ind, _)| *ind
					children=move |(ind, (show_candidate, candidate))| {
						view! {
							<Show when=move || show_candidate.get()>
								<div style="margin:5px">
									<label for=move || format!("candidate_{}", ind)>{move || format!("Candidate {}:", ind+1)}</label>
									<input
										id=move || format!("candidate_{}", ind)
										type="text"
										name=move || format!("candidates[{}]", ind)
										prop:value=candidate
										on:input:target=move |event| {candidate.set(event.target().value()); if (event.target().value().len() > 0) && (ind==(candidates.read_untracked().len()-1)) {candidates.write().push((RwSignal::new(true), RwSignal::new(String::new())))}}
									/>
									<Show when=move || (ind > 1)>
										<button on:click=move |_| {show_candidate.set(false)}>
											<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="black" viewBox="0 0 16 16">
												<path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
											</svg>
										</button>
									</Show>
								</div>
							</Show>
						}
					}
				/>
				<input type="submit" on:click:target=move |event| {let _ = event.target().form().expect("form to be connected").request_submit();event.target().set_disabled(true); event.target().set_value("Submitting…");}/>
			</ActionForm>
		</div>
	}
}

#[component]
fn VoteParent() -> impl IntoView {
	view! {
		<Outlet/>
	}
}

#[derive(Params, PartialEq)]
struct AdminPageParams {
	election_id: Option<String>,
	admin_id: Option<String>,
}

#[component]
fn AdminPage() -> impl IntoView {
	let params = use_params::<AdminPageParams>();

	let election_id = move || {
		params
			.read_untracked()
			.as_ref()
			.ok()
			.and_then(|params| params.election_id.clone())
			.unwrap_or_default()
	};
	let admin_id = move || {
		params
			.read_untracked()
			.as_ref()
			.ok()
			.and_then(|params| params.admin_id.clone())
			.unwrap_or_default()
	};
	let election_info = OnceResource::new(endpoints::get_election_admin_info(election_id(), admin_id()));
	let location = move || window().location().origin().expect("no location found on window");

	view! {
		<Suspense
			fallback=move || view! { <p>"Fetching election details..."</p> }
		>
			<h1>"Admin page for '" {Suspend::new(async move {election_info.await.unwrap().election_name})} "'"</h1>
			<div style="text-align: center">
				<div style="display: inline-block;text-align: left">
					<p>
						"admin url: "
						<a
							href={Suspend::new(async move {location() + "/" + &election_info.await.unwrap().admin_path})}
						>
						{location()} "/" {Suspend::new(async move {election_info.await.unwrap().admin_path})}
						</a>
					</p>
					<p>
						"vote url: "
						<a
							href={Suspend::new(async move {location() + "/" + &election_info.await.unwrap().vote_path})}
						>
						{location()} "/" {Suspend::new(async move {election_info.await.unwrap().vote_path})}
						</a>
					</p>
					<p>
						"results url: "
						<a
							href={Suspend::new(async move {location() + "/" + &election_info.await.unwrap().result_path})}
						>
						{location()} "/" {Suspend::new(async move {election_info.await.unwrap().result_path})}
						</a>
					</p>
				</div>
			</div>
		</Suspense>
	}
}

#[derive(Params, PartialEq)]
struct VotePageParams {
	election_id: Option<String>,
}

#[cfg(feature = "hydrate")]
// #[wasm_bindgen(module = "/Sortable.js")]
#[wasm_bindgen()]
extern "C" {
	type Sortable;

	#[wasm_bindgen(constructor)]
	fn new(element_id: String) -> Sortable;
}
#[cfg(feature = "ssr")]
struct Sortable;
#[cfg(feature = "ssr")]
impl Sortable {
	fn new(element_id: String) -> Sortable {
		let _ = element_id;
		Sortable {}
	}
}

#[component]
fn VotePage() -> impl IntoView {
	let params = use_params::<VotePageParams>();

	let election_id = move || {
		params
			.read()
			.as_ref()
			.ok()
			.and_then(|params| params.election_id.clone())
			.unwrap_or_default()
	};
	let election_info = OnceResource::new(endpoints::get_election_vote_info(election_id()));

	let list_loaded = RwSignal::new(false);
	let list_sortable = RwSignal::new(false);
	Effect::new(move |_| {
		if list_loaded.get() && !list_sortable.get() {
			console_log("applying Sortable");
			Sortable::new("example1".to_string());
			list_sortable.set(true);
		}
	});
	let cast_vote = ServerAction::<endpoints::CastVote>::new();

	view! {
		<Script src="/Sortable.js"></Script>
		<Suspense
			fallback=move || view! { <p>"Fetching election details..."</p> }
		>
			<h1>"Voting page for '"{Suspend::new(async move {election_info.await.unwrap().election_name})}"'"</h1>
			<div style="display: inline-block">
				{
					move || {
						election_info.get().map(
							|info| view! {

								<ActionForm action=cast_vote>
									<input type="hidden" id="election_id" name="election_id" value=election_id/>
									<div
										id="example1"
										class="list-group col"
										style="width:fit-content"
										on:mouseover=move |_| {list_loaded.set(true)}
										on:touchstart=move |_| {list_loaded.set(true)}
									>
										{
											info.expect("failed to fetch candidates").candidates.iter().enumerate().map(|(ind, cand)| view!{
												<div class="list-group-item">
													<input
														type="text"
														class="inert-text-input"
														name=move || format!("candidates[{}]", ind)
														prop:value={cand.clone()}
														readonly
													/>
													<div class="checkbox-wrapper">
														<input
															id=move || format!("comp_{}", ind)
															class="comp-checkbox"
															type="checkbox"
															name=move || format!("comp[{}]", ind)
															checked
															prop:value=move || format!("comp_{}", ind)
														/>
														<label for=move || format!("comp_{}", ind)></label>
													</div>
												</div>
											}).collect_view()
										}
									</div>
									<input
										type="submit"
										style="margin-top: 20px"
										on:click:target=move |event| {let _ = event.target().form().expect("form to be connected").request_submit();event.target().set_disabled(true); event.target().set_value("Submitting…");}
									/>
								</ActionForm>
							}
						)
					}
				}
			</div>
		</Suspense>
	}
}

#[component]
fn VoteThanksPage() -> impl IntoView {
	view! {
		<h1>"Thank you for voting!"</h1>
		<div style="text-align: center">
			<p>"If the vote administrator has shared the results url with you, you can view the results there."</p>
		</div>
	}
}

#[derive(Params, PartialEq)]
struct ResultPageParams {
	election_id: Option<String>,
	result_id: Option<String>,
}

#[component]
fn ResultPage() -> impl IntoView {
	let params = use_params::<ResultPageParams>();

	let election_id = move || {
		params
			.read_untracked()
			.as_ref()
			.ok()
			.and_then(|params| params.election_id.clone())
			.unwrap_or_default()
	};
	let result_id = move || {
		params
			.read_untracked()
			.as_ref()
			.ok()
			.and_then(|params| params.result_id.clone())
			.unwrap_or_default()
	};
	let election_results = OnceResource::new(endpoints::get_election_results(election_id(), result_id()));

	view! {
		<Suspense
			fallback=move || view! { <p>"Fetching election results..."</p> }
		>
			{
				Suspend::new(
					async move {
						let results = election_results.await.unwrap();
						let ranked_candidates = results.ranked_candidates.clone();
						view! {
							<h1>"Results page for '" {results.election_name} "'"</h1>
							<div style="text-align: center">
								<h4>"Number of votes: " {results.n_votes}</h4>
								<h4>"Defeats matrix:"</h4>
								<table class="defeat-matrix">
									<thead>
										<tr>
											<th scope="col"></th>
											{
												results.candidates.iter().map(
													|cand| view!{
														<th scope="col"><span>{cand.clone()}</span></th>
													}
												).collect_view()
											}
										</tr>
									</thead>
									<tbody>
										{
											results.defeats_matrix.iter().enumerate().map(
												|(row_ind, defeats_row)| view!{
													<tr>
														<th scope="row">{results.candidates[row_ind].clone()}</th>
														{
															defeats_row.iter().map(
																|defeat| view! {
																	<td>{*defeat}</td>
																}
															).collect_view()
														}
													</tr>
												}
											).collect_view()
										}
									</tbody>
								</table>
								<h4>"Global ranks:"</h4>
								<table class="ranks">
									<thead>
										<tr>
											<th scope="col">"rank"</th>
											<th scope="col">"candidate(s)"</th>
										</tr>
									</thead>
									<tbody>
										{
											move || {
												let ranked_candidates = results.ranked_candidates.clone();
												ranked_candidates.iter().enumerate().map(
													|(rank, candidates)| view!{
														<tr>
															<th scope="row">{rank}</th>
															<td>{candidates.join(" / ")}</td>
														</tr>
													}
												).collect_view()
											}
										}
									</tbody>
								</table>
								{
									move || {
										if ranked_candidates[0].len() > 1 {
											view! {
												<h4>"No winner, tied between: " {ranked_candidates[0].join(", ")}</h4>
											}
										} else {
											view! {
												<h4>"Winner: " {ranked_candidates[0][0].clone()}</h4>
											}
										}
									}
								}
							</div>
						}
					}
				)
			}
		</Suspense>
	}
}
