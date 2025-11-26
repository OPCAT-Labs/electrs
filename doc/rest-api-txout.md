# Transaction Output Endpoint

This document describes the transaction output lookup endpoint in the Electrs REST API.

## Overview

The transaction output endpoint allows you to retrieve the script and value (satoshi amount) for a specific transaction output (TxOut) by providing the transaction ID (txid) and output index (vout).

## Endpoint

### GET /tx/:txid/out/:vout

Returns the script and value for a specific transaction output.

## Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `txid` | string (hex) | Yes | Transaction ID (64-character hexadecimal string) |
| `vout` | integer | Yes | Output index (zero-based) |

## Response Format

Returns a JSON object containing the output details:

### Standard Version (without OPCAT Layer)

```json
{
  "scriptpubkey": "76a914...",
  "value": 100000
}
```

### OPCAT Layer Version

```json
{
  "scriptpubkey": "76a914...",
  "value": 100000,
  "data": "68656c6c6f"
}
```

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `scriptpubkey` | string | The output's scriptPubKey in hexadecimal format |
| `value` | integer | Amount in satoshis |
| `data` | string | (OPCAT Layer only) Hex-encoded data attached to the output |

## Behavior

### Transaction Lookup

The endpoint searches for the transaction output in the following order:

1. **Chain database** - Checks confirmed transactions first
2. **Mempool** - If not found in chain, checks unconfirmed transactions

### Cache TTL

The response cache duration depends on the transaction's confirmation status:

- **Confirmed transactions**: Long TTL (5 years) for deeply confirmed outputs
- **Recent confirmations**: Medium TTL based on confirmation depth
- **Mempool transactions**: Short TTL (60 seconds)

## Usage Examples

### Example 1: Get Output from Confirmed Transaction

```bash
curl "http://localhost:3000/tx/abc123def456.../out/0"
```

**Response**:
```json
{
  "scriptpubkey": "76a914fc7250a211deddc70ee5a2738de5f07817351cef88ac",
  "value": 100000
}
```

### Example 2: Get Output with OPCAT Layer Data

```bash
curl "http://localhost:3000/tx/abc123def456.../out/1"
```

**Response** (with OPCAT Layer enabled):
```json
{
  "scriptpubkey": "76a914fc7250a211deddc70ee5a2738de5f07817351cef88ac",
  "value": 250000,
  "data": "48656c6c6f20576f726c64"
}
```

### Example 3: Get Output from Mempool Transaction

```bash
curl "http://localhost:3000/tx/def789ghi012.../out/0"
```

**Response** (unconfirmed transaction):
```json
{
  "scriptpubkey": "0014751e76e8199196d454941c45d1b3a323f1433bd6",
  "value": 50000
}
```

## Error Responses

### 400 Bad Request

**Cause**: Invalid txid format or vout parameter.

**Response**:
```json
{
  "error": "Invalid txid or vout format"
}
```

### 404 Not Found

**Cause**: The specified output does not exist (transaction not found, or vout index out of range).

**Response**:
```json
{
  "error": "Output not found"
}
```

## Use Cases

### 1. Verify Output Before Spending

Check if an output exists and get its value before creating a transaction:

```javascript
async function verifyOutput(txid, vout) {
  try {
    const output = await fetch(
      `http://localhost:3000/tx/${txid}/out/${vout}`
    ).then(r => r.json());

    console.log(`Output exists with value: ${output.value} sats`);
    return output;
  } catch (err) {
    console.error('Output not found or invalid');
    return null;
  }
}
```

### 2. Decode ScriptPubKey Type

Retrieve the scriptPubKey to determine the output type:

```javascript
function getScriptType(scriptpubkey) {
  if (scriptpubkey.startsWith('76a914') && scriptpubkey.endsWith('88ac')) {
    return 'P2PKH';
  } else if (scriptpubkey.startsWith('0014')) {
    return 'P2WPKH';
  } else if (scriptpubkey.startsWith('a914') && scriptpubkey.endsWith('87')) {
    return 'P2SH';
  }
  return 'Unknown';
}

const output = await fetch('/tx/abc.../out/0').then(r => r.json());
console.log(`Script type: ${getScriptType(output.scriptpubkey)}`);
```

### 3. Validate UTXO Set

Verify that a specific output is unspent:

```javascript
async function isUnspent(txid, vout) {
  // First check if the output exists
  const output = await fetch(
    `http://localhost:3000/tx/${txid}/out/${vout}`
  ).then(r => r.json()).catch(() => null);

  if (!output) return false;

  // Then check if it's been spent
  const spend = await fetch(
    `http://localhost:3000/tx/${txid}/outspend/${vout}`
  ).then(r => r.json()).catch(() => null);

  return spend && !spend.spent;
}
```

## Performance Considerations

1. **Database Lookup**: The endpoint performs a single database lookup for confirmed transactions (very fast)
2. **Mempool Fallback**: If not found in chain, performs an in-memory mempool lookup (also very fast)
3. **Caching**: Responses are cached with appropriate TTL based on confirmation status
4. **Minimal Data**: Only essential fields are returned, minimizing response size

## Related Endpoints

This endpoint complements other transaction-related endpoints:

- `GET /tx/:txid` - Get full transaction details
- `GET /tx/:txid/status` - Get transaction confirmation status
- `GET /tx/:txid/outspend/:vout` - Check if a specific output has been spent
- `GET /tx/:txid/outspends` - Get spending status for all outputs

## Implementation Details

### Feature Flags

The response format changes based on the `opcat_layer` feature flag:

- **Without `opcat_layer`**: Returns `value` as `u64` (standard Bitcoin)
- **With `opcat_layer`**: Returns `value` via `.as_sat()` and includes additional `data` field

### Source Code

The endpoint is implemented in `/src/rest.rs` at line 1405-1440:

```rust
(&Method::GET, Some(&"tx"), Some(hash), Some(&"out"), Some(index), None) => {
    let txid = Txid::from_hex(hash)?;
    let vout = index.parse::<u32>()?;
    let outpoint = OutPoint { txid, vout };

    // Look up TxOut from chain or mempool
    let txout = query
        .chain()
        .lookup_txo(&outpoint)
        .or_else(|| {
            let mut outpoints = std::collections::BTreeSet::new();
            outpoints.insert(outpoint);
            query.mempool().lookup_txos(&outpoints).get(&outpoint).cloned()
        })
        .ok_or_else(|| HttpError::not_found("Output not found".to_string()))?;

    // Build response with conditional compilation for opcat_layer
    // ...
}
```

## Security Considerations

1. **Input Validation**: The endpoint validates txid format (must be valid 64-char hex) and vout (must be valid u32)
2. **DoS Protection**: Fast lookups prevent resource exhaustion
3. **No Authentication**: This is a read-only endpoint; no authentication required
4. **Rate Limiting**: Consider implementing rate limiting at the HTTP server level for production use

## See Also

- [REST API Schema Documentation](./schema.md)
- [Usage Guide](./usage.md)
- [UTXO Pagination](./rest-api-utxo-pagination.md)
