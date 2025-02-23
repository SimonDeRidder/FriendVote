use sea_orm_migration::{
	async_trait,
	prelude::Table,
	schema,
	sea_orm::{self, ColumnType, DeriveIden, DeriveMigrationName},
	sea_query, DbErr, MigrationTrait, SchemaManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Replace the sample below with your own migration scripts
		manager
			.create_table(
				Table::create()
					.table(Election::Table)
					.col(schema::string_len_uniq(Election::ElectionId, 16).primary_key())
					.col(schema::text(Election::Name))
					.col(schema::array(Election::Candidates, ColumnType::Text))
					.col(schema::string_len(Election::AdminId, 16))
					.col(schema::string_len(Election::ResultId, 16))
					.to_owned(),
			)
			.await?;
		manager
			.create_table(
				Table::create()
					.table(Votes::Table)
					.col(schema::pk_auto(Votes::VoteId))
					.col(schema::string_len(Votes::ElectionId, 16))
					.col(schema::array(Votes::CandOrder, ColumnType::Integer))
					.col(schema::array(Votes::CompIsBigger, ColumnType::Boolean))
					.to_owned(),
			)
			.await?;
		manager
			.create_index(
				sea_query::Index::create()
					.name("votes_election_index")
					.table(Votes::Table)
					.col(Votes::ElectionId)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Replace the sample below with your own migration scripts
		manager
			.drop_table(Table::drop().table(Election::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(Votes::Table).to_owned())
			.await
	}
}

#[derive(DeriveIden)]
enum Election {
	Table,
	ElectionId,
	Name,
	Candidates,
	AdminId,
	ResultId,
}

#[derive(DeriveIden)]
enum Votes {
	Table,
	VoteId,
	ElectionId,
	CandOrder,
	CompIsBigger,
}
