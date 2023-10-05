use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub type HeaderMapKey = String;
pub type HeaderMapValue = String;

#[derive(Clone, Debug)]
pub struct HeaderMap {
    map: RefCell<HashMap<HeaderMapKey, HeaderMapValue>>,
    iter_count: RefCell<Option<usize>>
}

impl Default for HeaderMap {
    fn default() -> Self {
        HeaderMap {
            map: RefCell::new(HashMap::new()),
            iter_count: RefCell::new(None)
        }
    }
}

impl HeaderMap {
    pub fn new() -> Self {
        HeaderMap::default()
    }
    
    pub fn insert(&self, k: HeaderMapKey, v: HeaderMapValue) -> Option<HeaderMapValue> {
        self.map.borrow_mut()
            .insert(k,v)
    }
    
    pub fn remove(&self, k: HeaderMapKey) -> Option<HeaderMapValue> {
        self.map.borrow_mut()
            .remove(&k)
    }
    
    pub fn current_iter_count(&self) -> Option<usize> {
        *self.iter_count.borrow()
    }
    
    pub fn len(&self) -> usize {
        self.map.borrow()
            .len()
    }
    
    pub fn insert_tuple(&self,tuple:(HeaderMapKey,HeaderMapValue)) -> Option<HeaderMapValue>{
        self.insert(tuple.0,tuple.1)
    }
    
    //  warn by clippy
    /*
        struct `HeaderMap` has a public `len` method, but no `is_empty` method
        Help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#len_without_is_empty
        Note: `#[warn(clippy::len_without_is_empty)]` on by default
    */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Iterator for HeaderMap {
    type Item = (HeaderMapKey, HeaderMapValue);
    
    fn next(&mut self) -> Option<Self::Item> {
        let is_none = self.iter_count.borrow().is_none();
        
        //未初始化
        if is_none {
            *self.iter_count.borrow_mut() = Some(0);
            //初始化了但是已经迭代完了
        } else if self.iter_count.borrow().unwrap() >= self.map.borrow().len() {
            *self.iter_count.borrow_mut() = None;           //改成None
            return None
        }
        
        let offset = self.iter_count.borrow().unwrap();
        //自增操作
        *self.iter_count.borrow_mut() = Some(offset + 1);
        
        self.map.borrow()
            .iter()
            .nth(offset)
            .map(|(ref_k, ref_v)| (ref_k.to_string(), ref_v.to_string()))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HeaderMappingError {
    UnknownChars,
    //不能转换为String
    UnknownString,
    //不能读取K,V
    EmptyRaw,
    //空buf
    EmptyString
    //空String
}

pub type HeaderMappingResult<T> = Result<T,HeaderMappingError>;

pub trait HeaderMappingType {
    fn parse_key_value(&self) -> HeaderMappingResult<(HeaderMapKey,HeaderMapValue)>;
}

impl HeaderMappingType for &[u8] {
    fn parse_key_value(&self) -> HeaderMappingResult<(HeaderMapKey, HeaderMapValue)> {
        if self.is_empty() {
            return Err(HeaderMappingError::EmptyRaw)
        }
        
        let str = String::from_utf8(self.to_vec());
        if str.is_err() {
            return Err(HeaderMappingError::UnknownChars)
        }
        
        let str = str.unwrap();
        if str.is_empty() {
            return Err(HeaderMappingError::EmptyString)
        }
        
        let str = str.replacen(':'," ",1);
        let mut sp = str.split_whitespace();
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
            .concat();
        
        Ok((key,value))
    }
}

impl HeaderMappingType for String{
    fn parse_key_value(&self) -> HeaderMappingResult<(HeaderMapKey, HeaderMapValue)> {
        if self.is_empty() {
            return Err(HeaderMappingError::EmptyString)
        };
        
        let str = self.replacen(':'," ",1);
        let mut sp = str.split_whitespace();
        
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
            .concat();
        
        Ok((key,value))
    }
}

impl HeaderMappingType for &str {
    fn parse_key_value(&self) -> HeaderMappingResult<(HeaderMapKey, HeaderMapValue)> {
        if self.is_empty() {
            return Err(HeaderMappingError::EmptyString)
        }
        
        let str = self.replacen(':'," ",1);
        let mut sp = str.split_whitespace();
        
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
            .concat();
        
        Ok((key,value))
    }
}

impl HeaderMap {
    pub fn try_insert<T>(&self,t:T) -> HeaderMappingResult<Option<HeaderMapValue>>
    where
        T:HeaderMappingType
    {
        Ok(self.insert_tuple(t.parse_key_value()?))
    }
}