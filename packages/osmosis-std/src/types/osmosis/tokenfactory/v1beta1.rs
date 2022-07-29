use osmosis_std_derive::CosmwasmExt;
/// MsgCreateDenom is the sdk.Msg type for allowing an account to create
/// a new denom. It requires a sender address and a subdenomination.
/// The (sender_address, sub_denomination) pair must be unique and cannot be
/// re-used. The resulting denom created is `factory/{creator
/// address}/{subdenom}`. The resultant denom's admin is originally set to be the
/// creator, but this can be changed later. The token denom does not indicate the
/// current admin.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom")]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenomResponse")]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag = "1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint
/// more of a token.  For now, we only support minting to the sender account
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgMintResponse")]
pub struct MsgMintResponse {}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.  For now, we only support burning from the sender account.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurn")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurnResponse")]
pub struct MsgBurnResponse {}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdmin")]
pub struct MsgChangeAdmin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdminResponse")]
pub struct MsgChangeAdminResponse {}
/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.DenomAuthorityMetadata")]
pub struct DenomAuthorityMetadata {
    /// Can be empty for no admin, or a valid osmosis address
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// Params holds parameters for the tokenfactory module
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub denom_creation_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsRequest")]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataRequest")]
pub struct QueryDenomAuthorityMetadataRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataResponse")]
pub struct QueryDenomAuthorityMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest")]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse")]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the tokenfactory module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.tokenfactory.v1beta1.GenesisDenom")]
pub struct GenesisDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
