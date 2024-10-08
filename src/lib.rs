use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct Hacker {
    pub name: String,
    pub description: String,
    pub team_name: String,
    pub key_image_url: Url,
}

#[blueprint]
mod hello {
    struct Hello {
        // Define what resources and data will be managed by Hello components
        manager: ResourceManager,
        end_date: Instant,
    }

    impl Hello {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_hello(
            owner_resource_address: ResourceAddress,
            dapp_def: ComponentAddress,
        ) -> Global<Hello> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Hello::blueprint_id());

            let global_caller_badge_rule = rule!(require(global_caller(component_address)));

            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let manager: ResourceManager = ResourceBuilder::new_ruid_non_fungible::<Hacker>(OwnerRole::Fixed(rule!(require(owner_resource_address))))
            .metadata(metadata! {
                init {
                    "name" => "Radix x EBC Hackathon 2024", updatable;
                    "symbol" => "HACK2024", updatable;
                    "info_url" => Url::of("https://www.radixdlt.com/"), updatable;
                    "icon_url" => Url::of("https://cdn.prod.website-files.com/6053f7fca5bf627283b582c2/6266da24f1cf78c68fb0c215_Radix-Icon-Transparent-400x400.png"), updatable;
                }
            }).mint_roles(mint_roles! {
                minter => global_caller_badge_rule.clone();
                minter_updater => rule!(require(owner_resource_address));
            })
            .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                non_fungible_data_updater => rule!(require(owner_resource_address));
                non_fungible_data_updater_updater => rule!(require(owner_resource_address));
            })
            .recall_roles(recall_roles! {
                recaller => rule!(require(owner_resource_address));
                recaller_updater => rule!(require(owner_resource_address));
            })
            .create_with_no_initial_supply();

            let current_time = Clock::current_time(TimePrecisionV2::Second);
            let end_date = current_time.add_days(7).unwrap();

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self { manager, end_date }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_resource_address))))
             
                .metadata(metadata! (
                    roles {
                        metadata_setter => rule!(deny_all);
                        metadata_setter_updater => rule!(deny_all);
                        metadata_locker => rule!(deny_all);
                        metadata_locker_updater => rule!(deny_all);
                    },
                    init {
                        "name" => "Hackathon Radix x EBC 2024".to_owned(), updatable;
                        "description" => "A POA for the Radix x EBC Hackathon 2024".to_owned(), updatable;
                        "dapp_definition" => dapp_def, updatable;
                        "icon_url" => Url::of("https://cdn.prod.website-files.com/6053f7fca5bf627283b582c2/6266da24f1cf78c68fb0c215_Radix-Icon-Transparent-400x400.png"), updatable;
                    }
                ))
              
                .with_address(address_reservation)
                .globalize()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn get_hackathon_badge(&mut self, team_name: String) -> Bucket {
            let current_time = Clock::current_time(TimePrecisionV2::Second);

            assert!(
                current_time.compare(self.end_date, TimeComparisonOperator::Lt),
                "The hackathon has ended, you can no longer claim your badge"
            );

            assert!(
                team_name.len() > 0,
                "Team name must be at least 1 character long"
            );

            let data = Hacker {
                name: "Radix x EBC Hackathon 2024".to_string(),
                description:
                    "Proof of attendance for participating in the Radix x EBC Hackathon 2024"
                        .to_string(),
                team_name,
                key_image_url: Url::of("https://dhnz2uhkq5om47mfpp3sxssguzwjgrr32j2b26erdhkunava4kuq.arweave.net/GdudUOqHXM59hXv3K8pGpmyTRjvSdB14kRnVRoKg4qk"),
            };

            self.manager.mint_ruid_non_fungible(data)
        }
    }
}
