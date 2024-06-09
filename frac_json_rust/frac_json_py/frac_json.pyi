from typing import Optional, List, Any

json_type = dict|list|str|int|float|bool|None

def encode(
    object: json_type,
    global_keys_table_bytes: Optional[bytes] = None,
    compression_level: Optional[int] = None,
    zstd_dict: Optional[bytes] = None,
) -> bytes:
    ...

def decode(
    frac_json_bytes: bytes,
    global_keys_table_bytes: Optional[bytes] = None,
    zstd_dict: Optional[bytes] = None,
) -> Any:
    ...

def keys_table_from_keys(keys: List[str]) -> bytes:
    ...

def keys_table_from_json(
    object: Any,
    max_count: Optional[int] = None,
    occurrence_cutoff: Optional[int] = None,
) -> bytes:
    ...
