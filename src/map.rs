use std::cell::RefCell;
use std::collections::HashMap;

pub type HTTPHeadKey = String;
pub type HTTPHeadValue = String;

#[derive(Clone, Debug)]
pub struct HTTPHeadMap {
    map: RefCell<HashMap<HTTPHeadKey, HTTPHeadValue>>,
    iter_count: RefCell<Option<usize>>
}

impl Default for HTTPHeadMap {
    fn default() -> Self {
        HTTPHeadMap {
            map: RefCell::new(HashMap::new()),
            iter_count: RefCell::new(None)
        }
    }
}

impl HTTPHeadMap {
    pub fn new() -> Self {
        HTTPHeadMap::default()
    }
    
    #[inline]
    pub fn insert(&self, k: HTTPHeadKey, v: HTTPHeadValue) -> Option<HTTPHeadValue> {
        self.map.borrow_mut()
            .insert(k, v)
    }
    
    pub fn remove(&self, k: HTTPHeadKey) -> Option<HTTPHeadValue> {
        self.map.borrow_mut()
            .remove(&k)
    }
    
    #[inline]
    pub fn current_iter_count(&self) -> Option<usize> {
        *self.iter_count.borrow()
    }
    
    #[inline]
    pub fn current_iter_count_mut(&self, value: Option<usize>) {
        *self.iter_count.borrow_mut() = value
    }
    
    #[inline]
    pub fn len(&self) -> usize {
        self.map.borrow()
            .len()
    }
    
    #[inline]
    pub fn insert_tuple(&self, tuple: (HTTPHeadKey, HTTPHeadValue)) -> Option<HTTPHeadValue> {
        self.insert(tuple.0, tuple.1)
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Iterator for HTTPHeadMap {
    type Item = (HTTPHeadKey, HTTPHeadValue);
    
    fn next(&mut self) -> Option<Self::Item> {
        let iter_count = self.current_iter_count();
        
        //未初始化
        if iter_count.is_none() {
            self.current_iter_count_mut(Some(0));
            //初始化了但是已经迭代完了
        } else if iter_count.unwrap() >= self.len() {
            self.current_iter_count_mut(None);         //改成None
            return None
        }
        
        let iter_count = self.current_iter_count().unwrap();
        //自增操作
        self.current_iter_count_mut(Some(iter_count + 1));
        
        self.map.borrow()
            .iter()
            .nth(iter_count)
            .map(|(ref_k, ref_v)| (ref_k.clone(), ref_v.clone()))
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

pub type HeaderMappingResult<T> = Result<T, HeaderMappingError>;

pub trait HeaderMappingType {
    fn parse_key_value(&self) -> HeaderMappingResult<(HTTPHeadKey, HTTPHeadValue)>;
}

impl HeaderMappingType for [u8] {
    fn parse_key_value(&self) -> HeaderMappingResult<(HTTPHeadKey, HTTPHeadValue)> {
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
        
        let str = str.replacen(':', " ", 1);
        let mut sp = str.split_whitespace();
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
                      .concat();
        
        Ok((key, value))
    }
}

impl HeaderMappingType for String {
    fn parse_key_value(&self) -> HeaderMappingResult<(HTTPHeadKey, HTTPHeadValue)> {
        if self.is_empty() {
            return Err(HeaderMappingError::EmptyString)
        };
        
        let str = self.replacen(':', " ", 1);
        let mut sp = str.split_whitespace();
        
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
                      .concat();
        
        Ok((key, value))
    }
}

impl HeaderMappingType for &str {
    fn parse_key_value(&self) -> HeaderMappingResult<(HTTPHeadKey, HTTPHeadValue)> {
        if self.is_empty() {
            return Err(HeaderMappingError::EmptyString)
        }
        
        let str = self.replacen(':', " ", 1);
        let mut sp = str.split_whitespace();
        
        if sp.clone().count() < 2 {
            return Err(HeaderMappingError::UnknownString)
        }
        
        let key = sp.next().unwrap().to_string();
        let value = sp.collect::<Vec<&str>>()
                      .concat();
        
        Ok((key, value))
    }
}

impl HTTPHeadMap {
    pub fn try_insert<T>(&self, t: T) -> HeaderMappingResult<Option<HTTPHeadValue>>
        where
            T: HeaderMappingType
    {
        Ok(self.insert_tuple(t.parse_key_value()?))
    }
}