#[macro_use]
extern crate log;

pub mod data_restore_driver;
pub mod events;
pub mod events_state;
pub mod genesis_state;
pub mod helpers;
pub mod rollup_ops;
pub mod storage_interactor;
pub mod tree_state;

// use std::str::FromStr;
use crate::data_restore_driver::DataRestoreDriver;
use clap::{App, Arg};
use server::ConfigurationOptions;
use storage::ConnectionPool;
use web3::types::{H160, H256};
use web3::Transport;

const ETH_BLOCKS_STEP: u64 = 1000;
const END_ETH_BLOCKS_OFFSET: u64 = 40;

fn main() {
    info!("Building Franklin accounts state");
    env_logger::init();
    let connection_pool = ConnectionPool::new();
    let config_opts = ConfigurationOptions::from_env();

    let cli = App::new("Data restore driver")
        .author("Matter Labs")
        .arg(
            Arg::with_name("genesis")
                .long("genesis")
                .help("Restores data with provided genesis (zero) block"),
        )
        .arg(
            Arg::with_name("continue")
                .long("continue")
                .help("Continues data restoreing"),
        )
        .get_matches();

    let mut driver = if cli.is_present("genesis") {
        create_data_restore_driver_with_genesis_acc(
            connection_pool,
            config_opts.web3_url.clone(),
            config_opts.contract_eth_addr.clone(),
            config_opts.contract_genesis_tx_hash.clone(),
            ETH_BLOCKS_STEP,
            END_ETH_BLOCKS_OFFSET,
        )
    } else {
        create_data_restore_driver_empty(
            connection_pool,
            // String::from("https://rinkeby.infura.io/v3/4406c3acf862426c83991f1752c46dd8"),
            // H160::from_str("d4047737804c4b9c6ceb7e8e051b42b249fafbf9").unwrap(),
            // H256::from_str("b99ebfea46cbe05a21cd80fe5597d97b204befc52a16303f579c607dc1ac2e2e").unwrap(),
            config_opts.web3_url.clone(),
            config_opts.contract_eth_addr.clone(),
            ETH_BLOCKS_STEP,
            END_ETH_BLOCKS_OFFSET,
        )
    }
    .expect("Cant load state");

    if cli.is_present("continue") {
        load_state_from_storage(&mut driver)
    }

    update_state(&mut driver);
}

pub fn create_data_restore_driver_empty(
    connection_pool: ConnectionPool,
    web3_url: String,
    contract_eth_addr: H160,
    contract_genesis_tx_hash: H256,
    eth_blocks_step: u64,
    end_eth_blocks_offset: u64,
) -> Result<DataRestoreDriver, failure::Error> {
    // let (_eloop, transport) = web3::transports::Http::new(&web3_url).unwrap();
    // let web3 = web3::Web3::new(transport);
    DataRestoreDriver::new_empty(
        connection_pool,
        web3_url,
        contract_eth_addr,
        contract_genesis_tx_hash,
        eth_blocks_step,
        end_eth_blocks_offset,
    )
}

/// Creates data restore driver state
///
/// # Arguments
///
/// * `connection_pool` - Database connection pool
///
pub fn create_data_restore_driver_with_genesis_acc(
    connection_pool: ConnectionPool,
    web3_url: String,
    contract_eth_addr: H160,
    contract_genesis_tx_hash: H256,
    eth_blocks_step: u64,
    end_eth_blocks_offset: u64,
) -> Result<DataRestoreDriver, failure::Error> {
    // let (_eloop, transport) = web3::transports::Http::new(&web3_url).unwrap();
    // let web3 = web3::Web3::new(transport);
    DataRestoreDriver::new_with_genesis_acc(
        connection_pool,
        web3_url,
        contract_eth_addr,
        contract_genesis_tx_hash,
        eth_blocks_step,
        end_eth_blocks_offset,
    )
}

/// Loads states from storage and start update
pub fn load_state_from_storage(driver: &mut DataRestoreDriver) {
    driver.load_state_from_storage().expect("Cant load state");
}

/// Runs states updates
///
/// # Arguments
///
/// * `driver` - DataRestore Driver config
///
pub fn update_state(driver: &mut DataRestoreDriver) {
    driver.run_state_update().expect("Cant update state");
}

pub fn stop_state_update(driver: &mut DataRestoreDriver) {
    driver.stop_state_update();
}
