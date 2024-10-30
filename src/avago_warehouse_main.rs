use crate::avago_warehouse_data::avago_warehouse_vault::AvagoWarehouseVault;
use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct OwnerNFTData {
    pub vault_address: Global<AvagoWarehouseVault>,
    pub owner_address: Global<Account>,
}

#[derive(ScryptoSbor, PartialEq)]
enum Step {
    Open,
    Proceed,
    Close,
}

#[derive(ScryptoSbor, NonFungibleData)]
struct VNFTData {
    pub vault_address: Global<AvagoWarehouseVault>,
}

#[blueprint]
mod avago_warehouse_main {

    struct AvagoWarehouseMain {
        owner: ResourceManager,
        vnft: ResourceManager,
        ticket: FungibleVault,
        step: Step,
        tx_id: Hash,
    }

    impl AvagoWarehouseMain {
        pub fn instantiate() -> Global<AvagoWarehouseMain> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(AvagoWarehouseMain::blueprint_id());

            let owner: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<OwnerNFTData>(OwnerRole::None)
                    .metadata(metadata! {
                        init {
                            "name" => "Avago Owner Nft", locked;
                            "symbol" => "AON", locked;
                        }
                    })
                    .mint_roles(mint_roles! {
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(deny_all);
                    })
                    .create_with_no_initial_supply();

            let vnft: ResourceManager =
                ResourceBuilder::new_ruid_non_fungible::<VNFTData>(OwnerRole::None)
                    .metadata(metadata! {
                        init {
                            "name" => "Avago Vault Nft", locked;
                            "symbol" => "AVN", locked;
                        }
                    })
                    .withdraw_roles(withdraw_roles! {
                        withdrawer => rule!(deny_all);
                        withdrawer_updater => rule!(require(global_caller(component_address)));
                    })
                    .deposit_roles(deposit_roles! {
                        depositor => rule!(deny_all);
                        depositor_updater => rule!(require(global_caller(component_address)));
                    })
                    .burn_roles(burn_roles! {
                        burner => rule!(allow_all);
                        burner_updater => rule!(deny_all);
                    })
                    .mint_roles(mint_roles! {
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(deny_all);
                    })
                    .create_with_no_initial_supply();

            let ticket: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Avago Vault Nft", locked;
                        "symbol" => "AVN", locked;
                    }
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(require(global_caller(component_address)));
                    withdrawer_updater => rule!(deny_all);
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(require(global_caller(component_address)));
                    depositor_updater => rule!(deny_all);
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                })
                .mint_initial_supply(1);

            Self {
                owner,
                vnft,
                step: Step::Close,
                tx_id: Runtime::transaction_hash(),
                ticket: FungibleVault::with_bucket(ticket),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        pub fn create_warehouse(
            &mut self,
            nft: Vec<NonFungibleBucket>,
            ft: Vec<FungibleBucket>,
            // third argument will be changed to avago NFT account in the future
            account: Global<Account>,
        ) -> Bucket {
            let address = AvagoWarehouseVault::instantiate(nft, ft);
            self.owner.mint_ruid_non_fungible(OwnerNFTData {
                vault_address: address,
                owner_address: account,
            })
        }

        pub fn open(&mut self, mode: String) -> FungibleBucket {
            assert!(
                self.tx_id != Runtime::transaction_hash(),
                "You can not call Open method twice."
            );
            self.tx_id = Runtime::transaction_hash();

            assert!(
                self.step == Step::Open,
                "Make sure that you call all methods sequentially"
            );
            self.step = Step::Proceed;

            match mode.as_str() {
                "withdraw" => {
                    self.vnft.set_withdrawable(rule!(allow_all));
                }
                "deposit" => {
                    self.vnft.set_depositable(rule!(allow_all));
                }
                _ => panic!("Invalid mode"),
            };

            self.ticket.take_all()
        }

        pub fn mint_vnft(&mut self, owner_nft: NonFungibleProof) -> Bucket {
            assert!(
                self.step == Step::Proceed,
                "Make sure that you call all methods sequentially"
            );
            self.step = Step::Close;

            let data: OwnerNFTData = owner_nft
                .check(self.owner.address())
                .non_fungible::<OwnerNFTData>()
                .data();

            self.vnft.mint_ruid_non_fungible(VNFTData {
                vault_address: data.vault_address,
            })
        }

        pub fn take_vnft_assets(
            &mut self,
            vnft: NonFungibleBucket,
        ) -> (Vec<NonFungibleBucket>, Vec<FungibleBucket>) {
            assert!(
                self.step == Step::Proceed,
                "Make sure that you call all methods sequentially"
            );
            self.step = Step::Close;

            vnft.non_fungible::<VNFTData>()
                .data()
                .vault_address
                .withdraw()
        }

        pub fn finish(&mut self, ticket: FungibleBucket) {
            assert!(
                self.step == Step::Close,
                "Make sure that you call all methods sequentially"
            );
            self.step = Step::Open;

            self.ticket.put(ticket);
            self.vnft.set_depositable(rule!(deny_all));
            self.vnft.set_withdrawable(rule!(allow_all));
        }
    }
}
