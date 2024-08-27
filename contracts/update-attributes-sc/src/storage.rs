use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view]
    #[storage_mapper("testTokenMapper")]
    fn test_token_mapper(&self) -> FungibleTokenMapper;
}
