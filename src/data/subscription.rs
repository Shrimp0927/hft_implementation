use serde;

// Subscribe struct tracks subscriptions for channels of the data stream
#[derive(Debug, serde::Deserialize)]
pub struct Subscription {
    #[serde(default)]
    quotes: Vec<String>,
    #[serde(default)]
    trades: Vec<String>,
    #[serde(default)]
    bars: Vec<String>,
}

impl Subscription {
    pub fn new() -> Self {
        Self { 
            quotes: Vec::new(), 
            trades: Vec::new(), 
            bars: Vec::new(),
        }
    }

    fn add_symbol(list: &mut Vec<String>, symbols: Vec<String>) {
        for symbol in symbols {
            list.push(symbol.clone());
        }
    }
    fn remove_symbol(list: &mut Vec<String>, symbols: Vec<String>) {
        let _ = list.retain(|item| !symbols.contains(item));
    }

    pub fn get_quotes(&self) -> &Vec<String> {
        &self.quotes
    }
    pub fn remove_quotes(&mut self, remove_symbols: Vec<String>) {
        let _ = Self::remove_symbol(&mut self.quotes, remove_symbols);
    }
    pub fn set_quotes(&mut self, add_symbols: Vec<String>) {
        let _ = Self::add_symbol(&mut self.quotes, add_symbols);
    }

    pub fn get_trades(&self) -> &Vec<String> {
        &self.trades
    }
    pub fn remove_trades(&mut self, remove_symbols: Vec<String>) {
        let _ = Self::remove_symbol(&mut self.trades, remove_symbols);
    }
    pub fn set_trades(&mut self, add_symbols: Vec<String>) {
        let _ = Self::add_symbol(&mut self.trades, add_symbols);
    }

    pub fn get_bars(&self) -> &Vec<String> {
        &self.bars
    }
    pub fn remove_bars(&mut self, remove_symbols: Vec<String>) {
        let _ = Self::remove_symbol(&mut self.bars, remove_symbols);
    }
    pub fn set_bars(&mut self, add_symbols: Vec<String>) {
        let _ = Self::add_symbol(&mut self.bars, add_symbols);
    }
}
