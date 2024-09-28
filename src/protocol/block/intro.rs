
// BlockPtr
StructFieldStruct!{ BlockPtr, 
	// ptr
	height : BlockHeight
	hash   : Hash
}


// BlockHead
StructFieldStruct!{ BlockHead, 
	// head
	version           : Uint1
	height            : BlockHeight
	timestamp         : Timestamp
	prevhash          : Hash
	mrklroot          : Hash
	transaction_count : Uint4
}


impl BlockHead {
	pub fn transaction_count(&self) -> &Uint4 {
		&self.transaction_count
	}
}


// BlockMeta
StructFieldStruct!{ BlockMeta, 
	// meta
	nonce         : Uint4      // Mining random value
	difficulty    : Uint4      // Target difficulty value
	witness_stage : Fixed2     // Witness quantity level
}


// BlockHead&Meta
StructFieldStruct!{ BlockHeadMeta, 
	// head                   
	head : BlockHead
	meta : BlockMeta
}


impl BlockRead for BlockHeadMeta {

    fn hash(&self) -> Hash {
        let intro = vec![ self.head.serialize(), self.meta.serialize() ].concat();
        let hx = x16rs::block_hash(self.height().to_u64(), intro);
        Hash::must(&hx[..])
    }

    fn version(&self) -> &Uint1 {
        &self.head.version
	}

    fn height(&self) -> &BlockHeight {
        &self.head.height
    }

    fn timestamp(&self) -> &Timestamp {
        &self.head.timestamp
    }

    fn prevhash(&self) -> &Hash {
        &self.head.prevhash
    }

    fn mrklroot(&self) -> &Hash {
        &self.head.mrklroot
    }

	fn transaction_count(&self) -> &Uint4 {
		self.head.transaction_count()
	}


	fn nonce(&self) -> &Uint4 {
        &self.meta.nonce
	}
	fn difficulty(&self) -> &Uint4 {
        &self.meta.difficulty
	}


}


impl BlockHeadMeta {

    pub fn set_mrklroot(&mut self, hx: Hash) {
        self.head.mrklroot = hx;
    }

}

pub type BlockIntro = BlockHeadMeta;

