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

struct Transaction;
struct Block;
struct ProofOfWork;

struct BcNode {
    transactions: Vec<Transaction>,
    blocks: Vec<Block>,
}

impl BcNode {
    pub fn new() -> Self {
        BcNode {
            transactions: vec![],
            blocks: vec![],
        }
    }

    pub fn start() {
        // begin listening for transactions and working on block (on a separate thread?)
        let (sender, receiver) = std::sync::mpsc::channel::<ProofOfWork>();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            sender.send(ProofOfWork {}).unwrap();
        });

        loop {
            match receiver.try_recv() {
                Ok(pow) => {
                    // broadcast to all other peers over network
                }
                Err(e) => {
                    // not sure what to do now
                }
            }

            // listen for block or transactions incoming from peers over the network
        }
    }

    pub fn collect(&mut self, t: Transaction) {
        // execute in response to receiving a transaction from a peer
        // verify and  include this transaction in the current block
        self.transactions.push(t);
        // continue with working on block (proof of work)
    }

    pub fn accept(&mut self, b: Block) {
        // execute in response to receiving a block from a peer
        // verify this block and update the local blocks accordingly
        self.blocks.push(b);
        // continue with working on block (proof of work)
    }
}
