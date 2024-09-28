
#[derive(Clone)]
pub struct EngineConf {
    pub max_block_txs: usize,
    pub max_block_size: usize,
    pub chain_id: u64, // sub chain id
    pub unstable_block: u64, // The number of blocks that are likely to fall back from the fork
    pub fast_sync: bool,
    pub data_dir: String,
    pub store_data_dir: PathBuf, // block data
    pub state_data_dir: PathBuf, // chain state
    //
    pub recent_blocks: bool,
    pub average_fee_purity: bool,
    // HAC miner
    pub miner_enable: bool,
    pub miner_reward_address: Address,
    pub miner_message: StringTrim16,
    // Diamond miner
    pub dmer_enable: bool,
    pub dmer_reward_address: Address,
    pub dmer_bid_account: Account,
    pub dmer_bid_min:  Amount,
    pub dmer_bid_max:  Amount,
    pub dmer_bid_step: Amount,
}


impl EngineConf {

    pub fn is_open_miner(&self) -> bool {
        self.miner_enable || self.dmer_enable
    }
    
    pub fn new(ini: &IniObj, dbv: u32) -> EngineConf {
    
        // datadir
        let data_path = get_data_path(ini);
    
        let mut state_data_dir = join_path(&data_path, "state");
        state_data_dir.push(format!("v{}", dbv));

        // server sec
        let sec_server = &ini_section(ini, "server");

        let mut cnf = EngineConf{
            max_block_txs: 1000,
            max_block_size: 1024*1024*1, // 1MB
            chain_id: 0,
            unstable_block: 4, // 4 block
            fast_sync: false,
            store_data_dir: join_path(&data_path, "store"),
            state_data_dir: state_data_dir,
            data_dir: data_path.to_str().unwrap().to_owned(),
            recent_blocks: ini_must_bool(sec_server, "recent_blocks", false),
            average_fee_purity: ini_must_bool(sec_server, "average_fee_purity", false),
            // HA Cminer
            miner_enable: false,
            miner_reward_address: Address::default(),
            miner_message: StringTrim16::default(),
            // Diamond miner
            dmer_enable: false,
            dmer_reward_address: Address::default(),
            dmer_bid_account: Account::create_by_password("123456").unwrap(),
            dmer_bid_min:  Amount::new_coin(1),
            dmer_bid_max:  Amount::new_coin(11),
            dmer_bid_step: Amount::new_small(5, 247),
        };

        let sec = &ini_section(ini, "node");
        cnf.fast_sync = ini_must_bool(sec, "fast_sync", false);

        let sec_mint = &ini_section(ini, "mint");
        cnf.chain_id = ini_must_u64(sec_mint, "chain_id", 0);

        // HAC miner
        let sec_miner = &ini_section(ini, "miner");
        cnf.miner_enable = ini_must_bool(sec_miner, "enable", false);
        if cnf.miner_enable {
            cnf.miner_reward_address = ini_must_address(sec_miner, "reward");
            let mut msg = ini_must_maxlen(sec_miner, "message", "", 16);
            let mut msgapp = vec![' ' as u8].repeat(16-msg.len());
            let msg: [u8; 16] = vec![msg.as_bytes().to_vec(), msgapp].concat().try_into().unwrap();
            cnf.miner_message = StringTrim16::from_readable(&msg);
        }

        // Diamond miner
        let sec_dmer = &ini_section(ini, "diamondminer");
        cnf.dmer_enable = ini_must_bool(sec_dmer, "enable", false);
        if cnf.dmer_enable {
            cnf.dmer_reward_address = ini_must_address(sec_dmer, "reward");
            cnf.dmer_bid_account = ini_must_account(sec_dmer, "bid_password");
            cnf.dmer_bid_min = ini_must_amount(sec_dmer, "bid_min").compress(4, true).unwrap();
            cnf.dmer_bid_max = ini_must_amount(sec_dmer, "bid_max").compress(4, true).unwrap();
            cnf.dmer_bid_step = ini_must_amount(sec_dmer, "bid_step").compress(4, true).unwrap();
        }


        // ok
        cnf
    }
    
}
