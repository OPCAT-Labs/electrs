# UTXO Endpoints - Pagination Support

This document describes the pagination functionality for UTXO endpoints in the Electrs REST API.

## Overview

The UTXO endpoints now support cursor-based pagination to handle addresses with large numbers of unspent transaction outputs (UTXOs) without raising `TooManyUtxos` errors.

## Endpoints

### GET /address/:address/utxo
### GET /scripthash/:hash/utxo

Returns the list of unspent transaction outputs (UTXOs) for a given address or script hash.

## Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `after_txid` | string (hex) | No | - | Cursor: Transaction ID to start pagination from |
| `after_vout` | integer | No | - | Cursor: Output index (vout) to start pagination from |
| `max_utxos` | integer | No | 50 | Maximum number of UTXOs to return per page |

### Parameter Rules

- Both `after_txid` and `after_vout` must be provided together, or neither
- Providing only one will return a `400 Bad Request` error
- The cursor (`after_txid:after_vout`) must exist (in mempool or chain), otherwise returns `422 Unprocessable Entity`
- `max_utxos` determines the page size (default: 50, uses `rest_default_max_mempool_txs` config)

## Response Format

Returns a JSON array of UTXO objects:

```json
[
  {
    "txid": "abc123...",
    "vout": 0,
    "status": {
      "confirmed": true,
      "block_height": 12345,
      "block_hash": "000000...",
      "block_time": 1234567890
    },
    "value": 100000,
    "data": "68656c6c6f"  // OPCAT Layer only: hex-encoded data
  },
  ...
]
```

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `txid` | string | Transaction ID |
| `vout` | integer | Output index |
| `status` | object | Confirmation status |
| `status.confirmed` | boolean | Whether the UTXO is confirmed |
| `status.block_height` | integer | Block height (if confirmed) |
| `status.block_hash` | string | Block hash (if confirmed) |
| `status.block_time` | integer | Block timestamp (if confirmed) |
| `value` | integer | Amount in satoshis |
| `data` | string | (OPCAT Layer only) Hex-encoded data attached to the output |

## Pagination Behavior

### Sorting

UTXOs are returned in the following deterministic order:

1. **Unconfirmed UTXOs first** (from mempool)
2. **By block height** (descending - newest blocks first)
3. **By transaction ID** (descending)
4. **By vout** (descending)

This ensures consistent pagination results even when new blocks are mined.

### Cursor Logic

- The cursor points to a specific UTXO (`txid:vout`)
- Results start **after** the cursor (cursor itself is excluded)
- If the cursor is not found in the result set, an empty array or the remaining items are returned
- Mempool UTXOs are checked first, then chain UTXOs

## Usage Examples

### Example 1: Get First Page (Default)

```bash
curl "http://localhost:3000/address/bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh/utxo"
```

Returns the first 50 UTXOs for the address.

### Example 2: Get First Page (Custom Size)

```bash
curl "http://localhost:3000/address/bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh/utxo?max_utxos=100"
```

Returns the first 100 UTXOs.

### Example 3: Get Next Page

Assuming the last UTXO from the previous response was:
```json
{
  "txid": "abc123...",
  "vout": 2,
  ...
}
```

Request the next page:
```bash
curl "http://localhost:3000/address/bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh/utxo?after_txid=abc123...&after_vout=2&max_utxos=100"
```

### Example 4: Using Script Hash

```bash
curl "http://localhost:3000/scripthash/8b01df4e368ea28f8dc0423bcf7a4923e3a12d307c875e47a0cfbf90b5c39161/utxo?max_utxos=50"
```

## Error Responses

### 400 Bad Request

**Cause**: Only one of `after_txid` or `after_vout` was provided.

**Response**:
```json
{
  "error": "after_txid requires after_vout parameter"
}
```
or
```json
{
  "error": "after_vout requires after_txid parameter"
}
```

### 422 Unprocessable Entity

**Cause**: The provided cursor (`after_txid:after_vout`) does not exist.

**Response**:
```json
{
  "error": "after_txid:after_vout not found"
}
```

## Backward Compatibility

- **No parameters**: Works exactly as before, returns all UTXOs up to the configured limit
- **Legacy behavior**: If you don't use pagination parameters, the endpoint behavior is unchanged
- **No breaking changes**: Existing clients continue to work without modification

## Implementation Details

### Removed Hard Limit

Previously, addresses with more than `utxos_limit` (default: 500) UTXOs would raise a `TooManyUtxos` error. This hard limit has been **removed**. Instead:

- The limit is now used as the **default page size** for pagination
- Users can paginate through unlimited UTXOs
- No error is raised for addresses with many UTXOs

### Configuration

The default `max_utxos` value is controlled by:
- Config parameter: `rest_default_max_mempool_txs` (default: 50)
- Can be overridden per request via the `max_utxos` query parameter

### Performance

- Pagination reduces memory usage by limiting the number of UTXOs loaded at once
- Deterministic sorting ensures consistent results across requests
- Results are fetched from both mempool and chain efficiently

## Best Practices

1. **Always use pagination** for addresses that might have many UTXOs
2. **Use reasonable page sizes** (50-100) for optimal performance
3. **Handle the last page** by checking if returned UTXOs < `max_utxos`
4. **Store the cursor** (last `txid:vout`) for fetching the next page
5. **Implement retry logic** for `422` errors if the cursor becomes invalid (e.g., due to reorg)

## Migration Guide

### Before (No Pagination)

```javascript
// Old code - might fail with TooManyUtxos error
const utxos = await fetch('/address/bc1q.../utxo').then(r => r.json());
```

### After (With Pagination)

```javascript
// New code - handles any number of UTXOs
async function getAllUtxos(address) {
  let allUtxos = [];
  let cursor = null;

  while (true) {
    const params = new URLSearchParams({ max_utxos: 100 });
    if (cursor) {
      params.append('after_txid', cursor.txid);
      params.append('after_vout', cursor.vout);
    }

    const utxos = await fetch(
      `/address/${address}/utxo?${params}`
    ).then(r => r.json());

    if (utxos.length === 0) break;

    allUtxos.push(...utxos);

    // Check if we've reached the last page
    if (utxos.length < 100) break;

    // Set cursor to last UTXO for next iteration
    const last = utxos[utxos.length - 1];
    cursor = { txid: last.txid, vout: last.vout };
  }

  return allUtxos;
}
```

## Related Endpoints

This pagination pattern is consistent with the transaction history endpoints:

- `/address/:address/txs` - Supports `after_txid` and `max_txs` parameters
- `/scripthash/:hash/txs` - Supports `after_txid` and `max_txs` parameters

## See Also

- [REST API Schema Documentation](./schema.md)
- [Usage Guide](./usage.md)
- [Transaction History Pagination](./rest-api-txs-pagination.md) (if exists)
