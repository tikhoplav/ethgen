use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Method {
    #[serde(rename = "eth_chainId")]
    EthChainId,
    #[serde(rename = "eth_gasPrice")]
    EthGasPrice,
    #[serde(rename = "eth_blockNumber")]
    EthBlockNumber,
    #[serde(rename = "eth_getBalance")]
    EthGetBalance,
    #[serde(rename = "eth_sendRawTransaction")]
    EthSendRawTransaction,
    #[serde(rename = "eth_call")]
    EthCall,
    #[serde(rename = "eth_estimateGas")]
    EthEstimateGas,
    #[serde(rename = "eth_getTransactionByHash")]
    EthGetTransactionByHash,
    #[serde(rename = "eth_getTransactionReceipt")]
    EthGetTransactionReceipt,
}

/// RPC Request
#[derive(Debug, PartialEq, Serialize)]
pub struct Request<T> {
    jsonrpc: super::Version,
    method: Method,
    params: T,
    id: u64, // TODO: make it U256
}

#[cfg(test)]
mod test {
    use super::super::Version;
    use super::*;
    use serde_json_core;

    #[test]
    fn request_into_json() {
        let rpc = Request {
            jsonrpc: Version::V2,
            method: Method::EthGasPrice,
            params: [0u8; 0],
            id: 1,
        };

        let mut buf = [0u8; 128];
        let n = serde_json_core::to_slice(&rpc, &mut buf).unwrap();
        let result = core::str::from_utf8(&buf[..n]).unwrap();

        let expected = r#"{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}"#;

        assert_eq!(expected, result);
    }
}
