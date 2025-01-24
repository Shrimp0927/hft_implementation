// Subscribe struct tracks subscriptions for channels of the data stream
pub struct Subscriptions<T> {
    quotes: Vec<T>,
    trades: Vec<T>,
    bars: Vec<T>,
}

impl<T: AsRef<str> + PartialEq + Clone> Subscriptions<T> {
    pub fn new() -> Self {
        Self { 
            quotes: Vec::new(), 
            trades: Vec::new(), 
            bars: Vec::new(),
        }
    }

    fn add_symbol(list: &mut Vec<T>, symbols: &[T]) {
        for symbol in symbols {
            list.push(symbol.clone());
        }
    }
    fn remove_symbol(list: &mut Vec<T>, symbols: &[T]) {
        let _ = list.retain(|item| !symbols.contains(item));
    }

    pub fn get_quotes(&self) -> &Vec<T> {
        &self.quotes
    }
    pub fn remove_quotes(&mut self, remove_symbols: &[T]) {
        let _ = Self::remove_symbol(&mut self.quotes, remove_symbols);
    }
    pub fn add_quotes(&mut self, add_symbols: &[T]) {
        let _ = Self::add_symbol(&mut self.quotes, add_symbols);
    }

    pub fn get_trades(&self) -> &Vec<T> {
        &self.trades
    }
    pub fn remove_trades(&mut self, remove_symbols: &[T]) {
        let _ = Self::remove_symbol(&mut self.trades, remove_symbols);
    }
    pub fn add_trades(&mut self, add_symbols: &[T]) {
        let _ = Self::add_symbol(&mut self.trades, add_symbols);
    }

    pub fn get_bars(&self) -> &Vec<T> {
        &self.bars
    }
    pub fn remove_bars(&mut self, remove_symbols: &[T]) {
        let _ = Self::remove_symbol(&mut self.bars, remove_symbols);
    }
    pub fn add_bars(&mut self, add_symbols: &[T]) {
        let _ = Self::add_symbol(&mut self.bars, add_symbols);
    }
}
