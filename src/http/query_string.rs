
use std::collections::HashMap;
use std::convert::From;


#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {

    pub fn get(&self, key: &str) -> Option<&Value> {

        self.data.get(key)
    }
    
}

impl<'buf> From<&'buf str> for QueryString<'buf> {

    fn from(s: &'buf str) -> Self {

        let mut data = HashMap::new();

        for sub_str in s.split('&') {

            let mut key = sub_str;
            let mut value = "";

            if let Some(value_index) = sub_str.find('=') {
                key = &sub_str[..value_index];
                value = &sub_str[value_index + 1..];
            }

            data.entry(key).and_modify(|existing_value| match existing_value {
                Value::Single(prev_value) => {
                    *existing_value = Value::Multiple(vec![prev_value, value]);
                },
                Value::Multiple(prev_vec) => prev_vec.push(value),
            })
            .or_insert(Value::Single(value));

        }

        QueryString { data }
    }
    
}

