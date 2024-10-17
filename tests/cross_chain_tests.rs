#[cfg(test)]
mod tests {
    use crate::cross_chain::{CrossChainBridge};

    #[test]
    fn test_cross_chain_transfer() {
        let bridge = CrossChainBridge::new("Nocoin".to_string(), "Ethereum".to_string());
        bridge.initiate_transfer("Alice", "Bob", 100);
        // Ensure that the transfer logic works and logs the expected result.
    }
}
