#![no_std]

use chrono::{DateTime, Months, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use thiserror_no_std::Error;
use chrono::serde::ts_seconds;
use postcard::to_allocvec;
use soroban_sdk::{Env, Vec};
use soroban_sdk::{contractimpl};

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Edunode<'a> {
    account_key: &'a str
}

#[derive(Serialize, Deserialize)]
pub struct Certificate<'a> {
    cert_id: &'a str,
    recipient_name: &'a str,
    issuing_institution: &'a str,
    course_name: &'a str,
    #[serde(with = "ts_seconds")]
    issue_date: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    expiry_date: DateTime<Utc>
}

#[derive(Error, Debug)]
pub enum EdunodeError {
    #[error(transparent)]
    PostcardError (#[from] postcard::Error),
}

impl Edunode<'_> {
    pub fn new(account_key: &str) -> Edunode {
        // Check API keys and connectivity here.

        Edunode {
            account_key
        }
    }

    /// Mint a certificate
    pub fn mint_certificate(&self, cert: &Certificate, env: &Env) -> Result<(), EdunodeError> {
        let output = match to_allocvec(cert) {
            Ok(v) => v,
            Err(e) => return Err(EdunodeError::PostcardError(e))
        };

        let mut soroban_vec = Vec::new(env);
        for byte in output {
            soroban_vec.push_back(byte as u32);
        }

        env.storage().set(&cert.cert_id, &soroban_vec);

        Ok(())
    }

    /// Verify a certificate. An error does not mean the certificate is invalid.
    pub fn verify_certificate(&self, cert: &Certificate) -> Result<bool, EdunodeError> {
        Ok(true)
    }
}

struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn test(env: Env) {
        // Not now, just some made up. (Can't get time from a no_std environment.)
        let now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);
        let then = now.checked_add_months(Months::new(12)).unwrap();

        let edunode = Edunode::new("foobar");

        let cert = Certificate {
            cert_id: "foo_id",
            recipient_name: "John Doe",
            issuing_institution: "ACME Corp",
            course_name: "EdunodeCourse",
            issue_date: now,
            expiry_date: then,
        };

        edunode.mint_certificate(&cert, &env).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Months, NaiveDateTime, TimeZone, Utc};
    use soroban_sdk::Env;
    use crate::{Certificate, Edunode, TestContract, TestContractClient};

    #[test]
    fn mint_cert() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TestContract);
        let client = TestContractClient::new(&env, &contract_id);

        client.test();
    }
}
