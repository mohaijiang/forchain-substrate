use codec::{Decode, Encode};
use sp_std::vec::Vec;
use frame_support::{Parameter, RuntimeDebug};
use sp_runtime::traits::AtLeast32BitUnsigned;

// /*
//     Use BondundedVec to Specify that the interpretation is up to 500 bytes
// */
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct AiModel<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub hash : Vec<u8>,
    pub name: Vec<u8>,
    pub link: Vec<u8>,
    pub images: Vec<Vec<u8>>,
    pub image_links: Vec<Vec<u8>>,
    pub download_price: u128,
    pub comment: Vec<u8>,
    pub account_id : AccountId,
    pub block_number : BlockNumber,
}

impl<BlockNumber, AccountId> AiModel<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub fn new( hash: Vec<u8>,
                name: Vec<u8>,
                link: Vec<u8>,
                images: Vec<Vec<u8>>,
                image_links: Vec<Vec<u8>>,
                download_price: u128,
                comment: Vec<u8>,
                account_id: AccountId,
                block_number: BlockNumber
    ) -> Self {
        Self{
            hash,
            name,
            link,
            images,
            image_links,
            download_price,
            comment,
            account_id,
            block_number,
        }
    }
}


#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo,PartialOrd, Ord)]
pub struct AiPost<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub model_hash : Vec<u8>,
    pub name: Vec<u8>,
    pub images: Vec<Vec<u8>>,
    pub image_links: Vec<Vec<u8>>,
    pub comment: Vec<u8>,
    pub account_id : AccountId,
    pub block_number : BlockNumber,
}

impl<BlockNumber, AccountId> AiPost<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub fn new( model_hash: Vec<u8>,
                name: Vec<u8>,
                images: Vec<Vec<u8>>,
                image_links: Vec<Vec<u8>>,
                comment: Vec<u8>,
                account_id: AccountId,
                block_number: BlockNumber
    ) -> Self {
        Self{
            model_hash,
            name,
            images,
            image_links,
            comment,
            account_id,
            block_number,
        }
    }
}