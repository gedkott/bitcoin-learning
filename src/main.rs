use openssl::{bn::BigNum, dsa, error::ErrorStack};

fn main() {
    let (owner1, owner2) = match (Owner::new(), Owner::new()) {
        (Ok(owner1), Ok(owner2)) => (owner1, owner2),
        _ => panic!("owner creation failed"),
    };

    println!("{:?} {:?}", owner1, owner2)
}

#[derive(Debug)]
struct Owner {
    public_key: BigNum,
    private_key: BigNum,
}

enum BitcoinError {
    KeyGeneration,
    OpenSslError(ErrorStack),
}

type BcResult<T> = Result<T, BitcoinError>;

impl Owner {
    pub fn new() -> BcResult<Self> {
        dsa::Dsa::generate(2048)
            .map_err(BitcoinError::OpenSslError)
            .and_then(|dsa| {
                let pub_key = dsa.pub_key().to_owned();
                let priv_key = dsa.pub_key().to_owned();
                match (pub_key, priv_key) {
                    (Ok(pub_key), Ok(priv_key)) => Ok((pub_key, priv_key)),
                    _ => Err(BitcoinError::KeyGeneration),
                }
            })
            .map(|(pub_key, priv_key)| Owner {
                public_key: pub_key,
                private_key: priv_key,
            })
    }
}

struct DigitalSignature {
    hash: String,
    owner_public_key: String,
    previous_owner_signature: Option<Box<DigitalSignature>>,
}

struct ElectronicCoin {
    sigs: Vec<DigitalSignature>,
}

impl ElectronicCoin {
    pub fn transfer() {
        // update sigs with digital signature of previous transaction hash
        // and next owner public key
    }

    pub fn verify() {
        // verify chain of signatures
    }
}
