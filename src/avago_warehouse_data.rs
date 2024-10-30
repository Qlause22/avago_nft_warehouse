use scrypto::prelude::*;

#[blueprint]
mod avago_warehouse_vault {
    struct AvagoWarehouseVault {
        nft_assets: Vec<NonFungibleVault>,
        ft_assets: Vec<FungibleVault>,
        is_took: bool,
        tx_id: Option<Hash>,
    }

    impl AvagoWarehouseVault {
        pub fn instantiate(
            nft: Vec<NonFungibleBucket>,
            ft: Vec<FungibleBucket>,
        ) -> Global<AvagoWarehouseVault> {
            let nft_assets = nft.into_iter().map(NonFungibleVault::with_bucket).collect();
            let ft_assets = ft.into_iter().map(FungibleVault::with_bucket).collect();

            Self {
                nft_assets,
                ft_assets,
                is_took: false,
                tx_id: None,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn withdraw(&mut self) -> (Vec<NonFungibleBucket>, Vec<FungibleBucket>) {
            assert!(!self.is_took, "contract already withdrawed");
            self.is_took = true;
            self.tx_id = Some(Runtime::transaction_hash());

            (
                self.nft_assets.iter_mut().map(|x| x.take_all()).collect(),
                self.ft_assets.iter_mut().map(|x| x.take_all()).collect(),
            )
        }
    }
}
