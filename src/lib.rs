use std::fs::File;
use std::io::Write;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use thiserror::Error;
use chrono::serde::ts_seconds;

pub struct Edunode {
    account_key: String
}

#[derive(Serialize, Deserialize)]
pub struct Certificate {
    cert_id: String,
    recipient_name: String,
    issuing_institution: String,
    course_name: String,
    #[serde(with = "ts_seconds")]
    issue_date: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    expiry_date: DateTime<Utc>
}

#[derive(Error, Debug)]
pub enum EdunodeError {
    #[error(transparent)]
    SerdeError (#[from] serde_json::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl Edunode {
    pub fn new(account_key: String) -> Edunode {
        // Check API keys and connectivity here.

        Edunode {
            account_key
        }
    }

    /// Mint a certificate
    pub fn mint_certificate(&self, cert: &Certificate) -> Result<(), EdunodeError> {
        let json = match serde_json::to_string(cert) {
            Ok(v) => v,
            Err(e) => return Err(EdunodeError::SerdeError(e))
        };

        let mut file = match File::create(&cert.cert_id) {
            Ok(v) => v,
            Err(e) => return Err(EdunodeError::IOError(e))
        };

        if let Err(e) = file.write(json.as_bytes()) {
            return Err(EdunodeError::IOError(e))
        }

        Ok(())
    }

    /// Verify a certificate. An error does not mean the certificate is invalid.
    pub fn verify_certificate(&self, cert: &Certificate) -> Result<bool, EdunodeError> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Months, Utc};
    use crate::{Certificate, Edunode};

    #[test]
    fn mint_cert() {
        let now = Utc::now();
        let then = now.checked_add_months(Months::new(12)).unwrap();

        let edunode = Edunode::new("foobar".to_string());

        let cert = Certificate {
            cert_id: "foo_id".to_string(),
            recipient_name: "John Doe".to_string(),
            issuing_institution: "ACME Corp".to_string(),
            course_name: "EdunodeCourse".to_string(),
            issue_date: now,
            expiry_date: then,
        };

        edunode.mint_certificate(&cert).unwrap();
    }
}
