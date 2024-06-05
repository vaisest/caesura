use crate::options::Options;

pub struct ValueProvider;

impl ValueProvider {
    pub fn get<TOptions: Options, TValue, F>(options: &TOptions, selector: F) -> TValue
    where
        F: FnOnce(&TOptions) -> Option<TValue>,
    {
        selector(options)
            .expect(format!("{} should have all values set", TOptions::get_name()).as_str())
    }
}
