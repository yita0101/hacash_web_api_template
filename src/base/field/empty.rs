
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Empty {
}


impl Serialize for Empty {

    fn serialize(&self) -> Vec<u8> {
        vec![]
    }

    fn size(&self) -> usize {
        0
    }

}


impl Parse for Empty {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Result<usize, Error> {
        Ok(seek)    
    }

}

impl Field for Empty {

}



///////////////////////



#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct VecWrap {
    pub data: Vec<u8>,
}


impl Serialize for VecWrap {

    fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

}
