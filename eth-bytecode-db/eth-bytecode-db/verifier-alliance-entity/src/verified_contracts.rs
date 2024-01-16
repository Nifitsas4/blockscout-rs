//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "verified_contracts")]
pub struct Model {
    pub id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deployment_id: Uuid,
    pub compilation_id: Uuid,
    pub creation_match: bool,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub creation_values: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub creation_transformations: Option<Json>,
    pub runtime_match: bool,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub runtime_values: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub runtime_transformations: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::compiled_contracts::Entity",
        from = "Column::CompilationId",
        to = "super::compiled_contracts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CompiledContracts,
    #[sea_orm(
        belongs_to = "super::contract_deployments::Entity",
        from = "Column::DeploymentId",
        to = "super::contract_deployments::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ContractDeployments,
}

impl Related<super::compiled_contracts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompiledContracts.def()
    }
}

impl Related<super::contract_deployments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContractDeployments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
