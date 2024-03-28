use crate::lmc_algo::algo::*;
use crate::security::Security;

pub(crate) mod algo;

pub fn lmc_algo(security: Security) -> Result<(), i8> {
    loop {
        if !is_lock_algo_can_activate() {
            return Err(1);
        } else if !security.is_active() {
            return Err(2);
        } else {
            match Result::Ok {

            }
            println!("can't active LMC algorithm in this hour ");
        }
    }
}