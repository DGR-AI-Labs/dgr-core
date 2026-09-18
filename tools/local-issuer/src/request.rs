//! Exact simulation invoice grammar and DGR-ACT2 commitment. Agent-authored T0.
use sha2::{Digest, Sha256};

pub const MAX_REQUEST: usize = 245;
pub const NAMES: [&str; 7] = [
    "action",
    "amount",
    "currency",
    "destination",
    "invoice_id",
    "source_account",
    "tool",
];
pub const TOOL: &str = "dgr.openclaw.invoice.record.v1";

pub struct ValidatedInvoice {
    values: [String; 7],
}
#[derive(Debug, PartialEq, Eq)]
pub struct RequestError;

pub fn parse_request(input: &[u8]) -> Result<ValidatedInvoice, RequestError> {
    if input.len() > MAX_REQUEST || !input.is_ascii() {
        return Err(RequestError);
    }
    let mut rest = input;
    let mut values: [String; 7] = std::array::from_fn(|_| String::new());
    for (i, name) in NAMES.iter().enumerate() {
        let prefix = format!("{}\"{}\":\"", if i == 0 { "{" } else { "," }, name);
        rest = rest.strip_prefix(prefix.as_bytes()).ok_or(RequestError)?;
        let end = rest.iter().position(|b| *b == b'"').ok_or(RequestError)?;
        values[i] = std::str::from_utf8(&rest[..end])
            .map_err(|_| RequestError)?
            .to_owned();
        rest = &rest[end + 1..];
    }
    if rest != b"}\n"
        || values[0] != "record_invoice"
        || values[2] != "USD"
        || values[3] != "simulation-vendor-1"
        || values[5] != "simulation-account-1"
        || values[6] != TOOL
    {
        return Err(RequestError);
    }
    let amount = values[1].as_bytes();
    if amount.is_empty()
        || amount.len() > 20
        || !amount.iter().all(u8::is_ascii_digit)
        || (amount.len() > 1 && amount[0] == b'0')
    {
        return Err(RequestError);
    }
    let invoice = values[4].strip_prefix("SIM-").ok_or(RequestError)?;
    if invoice.is_empty()
        || invoice.len() > 32
        || !invoice
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
    {
        return Err(RequestError);
    }
    Ok(ValidatedInvoice { values })
}
impl ValidatedInvoice {
    pub fn fields(&self) -> impl Iterator<Item = (&'static str, &str)> {
        NAMES
            .into_iter()
            .zip(self.values.iter().map(String::as_str))
    }
    pub fn action_commitment(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(b"DGR-ACT2\0");
        for (tag, value) in (1_u8..=7).zip(&self.values) {
            hash.update([tag]);
            // Validated values are at most 36 ASCII bytes.
            hash.update(
                u32::try_from(value.len())
                    .expect("bounded field")
                    .to_be_bytes(),
            );
            hash.update(value.as_bytes());
        }
        hash.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(amount: &str, invoice: &str) -> Vec<u8> {
        format!("{{\"action\":\"record_invoice\",\"amount\":\"{amount}\",\"currency\":\"USD\",\"destination\":\"simulation-vendor-1\",\"invoice_id\":\"{invoice}\",\"source_account\":\"simulation-account-1\",\"tool\":\"{TOOL}\"}}\n").into_bytes()
    }
    #[test]
    fn exact_grammar_boundaries() {
        for amount in ["0", "1", "18446744073709551616", "99999999999999999999"] {
            assert!(parse_request(&request(amount, "SIM-A")).is_ok());
        }
        for amount in [
            "",
            "00",
            "01",
            "+1",
            "-1",
            "1.0",
            "1e2",
            " 1",
            "999999999999999999999",
        ] {
            assert!(parse_request(&request(amount, "SIM-A")).is_err());
        }
        for id in ["SIM-", "sim-A", "SIM-a", "SIM-A/", "SIM-A\\n"] {
            assert!(parse_request(&request("1", id)).is_err());
        }
        let max = request("99999999999999999999", &format!("SIM-{}", "Z".repeat(32)));
        assert_eq!(max.len(), MAX_REQUEST);
        assert_eq!(
            crate::hex(&parse_request(&max).unwrap().action_commitment()),
            "89f5d9ac860ed8f405e47ebecb446a9cda24d5a381fee164cbb1e01de0045f7f"
        );
        for n in 0..max.len() {
            assert!(parse_request(&max[..n]).is_err());
        }
        let mut suffix = max.clone();
        suffix.push(b'\n');
        assert!(parse_request(&suffix).is_err());
        let input = request("1", "SIM-A");
        for (from, to) in [
            ("record_invoice", "pay_invoice"),
            ("USD", "EUR"),
            (TOOL, "other.tool"),
            ("{\"action", "{ \"action"),
            ("}\n", "}\r\n"),
        ] {
            let mutated = String::from_utf8(input.clone()).unwrap().replace(from, to);
            assert!(parse_request(mutated.as_bytes()).is_err());
        }
        assert_ne!(
            parse_request(&input).unwrap().action_commitment(),
            parse_request(&request("2", "SIM-A"))
                .unwrap()
                .action_commitment()
        );
    }
}
