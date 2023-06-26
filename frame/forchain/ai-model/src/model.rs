use codec::{Decode, Encode};
use sp_std::vec::Vec;
use frame_support::{Parameter, RuntimeDebug};
use sp_runtime::traits::AtLeast32BitUnsigned;

// /*
//    模型结构
// */
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct AiModel<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    // 模型hash
    pub hash : Vec<u8>,
    // 模型名
    pub name: Vec<u8>,
    // 模型下载地址
    pub link: Vec<u8>,
    // 模型的实例图片
    pub images: Vec<Vec<u8>>,
    // 模型的实例图片下载地址
    pub image_links: Vec<Vec<u8>>,
    // 购买（下载）模型的价格
    pub download_price: u128,
    // 模型的描述信息（markdown 格式）
    pub comment: Vec<u8>,
    // 模型拥有者账户地址
    pub account_id : AccountId,
    // 上传模型的区块
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


// /*
//    模型下用户Post（帖子）结构
// */
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo,PartialOrd, Ord)]
pub struct AiPost<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    // 帖子所属模型hash
    pub model_hash : Vec<u8>,
    // 帖子名
    pub name: Vec<u8>,
    // 帖子的图片列表
    pub images: Vec<Vec<u8>>,
    // 帖子的图片下载地址列表
    pub image_links: Vec<Vec<u8>>,
    // 帖子的描述信息（markdown 格式）
    pub comment: Vec<u8>,
    // 帖子创建者账户id
    pub account_id : AccountId,
    // 帖子的创建所在区块
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