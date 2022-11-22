//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "bytecode_type")]
pub enum BytecodeType {
    #[sea_orm(string_value = "creation_input")]
    CreationInput,
    #[sea_orm(string_value = "deployed_bytecode")]
    DeployedBytecode,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "verification_type")]
pub enum VerificationType {
    #[sea_orm(string_value = "flattened_contract")]
    FlattenedContract,
    #[sea_orm(string_value = "metadata")]
    Metadata,
    #[sea_orm(string_value = "multi_part_files")]
    MultiPartFiles,
    #[sea_orm(string_value = "standard_json")]
    StandardJson,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "source_type")]
pub enum SourceType {
    #[sea_orm(string_value = "solidity")]
    Solidity,
    #[sea_orm(string_value = "vyper")]
    Vyper,
    #[sea_orm(string_value = "yul")]
    Yul,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "part_type")]
pub enum PartType {
    #[sea_orm(string_value = "main")]
    Main,
    #[sea_orm(string_value = "metadata")]
    Metadata,
}
