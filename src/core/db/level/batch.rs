
pub struct Writebatch {
    // ptr: *mut leveldb_writebatch_t,
}

impl Drop for Writebatch {
    fn drop(&mut self) {
        unsafe {
            // leveldb_writebatch_destroy(self.ptr);
        }
    }
}


impl Writebatch {
    /// Create a new writebatch
    pub fn new() -> Writebatch {
        // let ptr = unsafe { leveldb_writebatch_create() };
        Writebatch {  }
    }

    // Batch a put operation
    pub fn put(&mut self, k: &[u8], value: &[u8]) {
        // unsafe {
        //     leveldb_writebatch_put(self.ptr,
        //         k.as_ptr() as *mut c_char,
        //         k.len() as size_t,
        //         value.as_ptr() as *mut c_char,
        //         value.len() as size_t);
        // }
    }

    // Batch a delete operation
    pub fn delete(&mut self, k: &[u8]) {
        unsafe {
            // leveldb_writebatch_delete(self.ptr,
            //     k.as_ptr() as *mut c_char,
            //     k.len() as size_t);
        }
    }


}



