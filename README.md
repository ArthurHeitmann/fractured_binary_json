# Fractured Binary JSON

A binary JSON encoding optimized for small storage size.

When the schema of JSON data is known ahead of time, storage size can be further optimized by storing object key names in a separate global keys table.

`frac_json` can be used as an abbreviation and file extension.

Format specification can be found [here](./format_specification.md).

## Usage

### Python

```bash
pip install frac_json
```

[Usage examples](./frac_json_rust/frac_json_py/README.md)

### Node.js

```bash
npm install @raiderb/frac_json
```

[Usage examples](./frac_json_rust/frac_json_node/README.md)

## Benchmarks

Comparison of different JSON encoding formats in combination with compression.

Tables generated with [benchmark](./frac_json_rust/benchmark/) Rust sub-project.

Zstd used with compression level 3 and pre trained dictionary size of 10 KB. (Other compression methods like gzip or deflate have similar compression levels, usually 0 - 4 percentage points smaller, but are 2x - 4x slower)

Takeaways at the end.

### Storage size for small JSON files (< 5 KB):

|                                   | MediaContent.json | reddit_comments_1.json | reddit_posts_1.json | TwitterTimeline.json | CouchDB4k.json  |
|-----------------------------------|-------------------|------------------------|---------------------|----------------------|-----------------|
| plain text                        | 100.0% (485.0 B)  | 100.0% (1.8 KB)        | 100.0% (3.6 KB)     | 100.0% (2.0 KB)      | 100.0% (3.8 KB) |
| MessagePack                       | 80.4%             | 80.2%                  | 81.7%               | 83.4%                | 80.1%           |
| Smile                             | 82.1%             | 80.9%                  | 82.6%               | 84.1%                | 84.9%           |
| Smile (+shared)                   | 66.4%             | 80.9%                  | 79.5%               | 83.0%                | 71.9%           |
| frac json                         | 71.1%             | 80.2%                  | 79.3%               | 82.0%                | 77.0%           |
| frac json (+global table)         | 55.3%             | 33.1%                  | 42.6%               | 39.7%                | 74.7%           |
|                                   |                   |                        |                     |                      |                 |
| plain text (+zstd)                | 56.5%             | 48.4%                  | 41.6%               | 44.8%                | 41.0%           |
| MessagePack (+zstd)               | 52.6%             | 49.2%                  | 43.2%               | 46.5%                | 43.4%           |
| Smile (+zstd)                     | 52.2%             | 50.2%                  | 43.5%               | 47.0%                | 43.5%           |
| Smile (+shared) (+zstd)           | 52.6%             | 50.2%                  | 43.6%               | 47.0%                | 43.8%           |
| frac json (+zstd)                 | 51.8%             | 49.3%                  | 43.3%               | 46.0%                | 43.3%           |
| frac json (+global table) (+zstd) | 38.6%             | 28.2%                  | 26.2%               | 27.5%                | 41.8%           |

### Storage size for big JSON files (> 100 KB):

|                                   | reddit_comments_100.json | reddit_posts_100.json | twitter.json      | citm_catalog.json | canada.json     | jeopardy.json    |
|-----------------------------------|--------------------------|-----------------------|-------------------|-------------------|-----------------|------------------|
| plain text                        | 100.0% (205.0 KB)        | 100.0% (381.0 KB)     | 100.0% (456.0 KB) | 100.0% (488.6 KB) | 100.0% (2.0 MB) | 100.0% (50.1 MB) |
| MessagePack                       | 81.8%                    | 82.9%                 | 86.0%             | 68.5%             | 51.2%           | 87.9%            |
| Smile                             | 82.2%                    | 83.5%                 | 86.4%             | 75.4%             | 64.7%           | 88.2%            |
| Smile (+shared)                   | 41.2%                    | 46.0%                 | 42.3%             | 37.8%             | 64.7%           | 51.2%            |
| frac json                         | 40.8%                    | 46.9%                 | 50.1%             | 37.3%             | 51.1%           | 66.6%            |
| frac json (+global table)         | 40.4%                    | 46.2%                 | 49.8%             | 27.5%             | 51.1%           | 66.6%            |
|                                   |                          |                       |                   |                   |                 |                  |
| plain text (+zstd)                | 13.0%                    | 14.9%                 | 8.8%              | 2.5%              | 27.4%           | 22.8%            |
| MessagePack (+zstd)               | 13.2%                    | 15.1%                 | 9.2%              | 2.6%              | 22.2%           | 23.7%            |
| Smile (+zstd)                     | 13.1%                    | 14.8%                 | 9.0%              | 2.6%              | 20.9%           | 23.1%            |
| Smile (+shared) (+zstd)           | 12.4%                    | 14.4%                 | 8.5%              | 2.3%              | 20.9%           | 21.8%            |
| frac json (+zstd)                 | 12.5%                    | 14.4%                 | 8.6%              | 2.4%              | 24.3%           | 22.4%            |
| frac json (+global table) (+zstd) | 12.2%                    | 14.1%                 | 8.4%              | 2.2%              | 24.3%           | 22.4%            |

### Average storage size across many small objects

|                                                 | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                      | 100.0% (1.7 KB)                         | 100.0% (3.9 KB)                      | 100.0% (4.6 KB)              | 100.0% (241.1 B)              |
| MessagePack                                     | 78.8%                                   | 82.7%                                | 86.0%                        | 88.3%                         |
| Smile                                           | 79.5%                                   | 83.5%                                | 86.5%                        | 90.3%                         |
| Smile (+shared)                                 | 79.1%                                   | 80.4%                                | 66.9%                        | 90.3%                         |
| frac json                                       | 78.8%                                   | 80.7%                                | 69.4%                        | 89.3%                         |
| frac json (+global table)                       | 30.1%                                   | 47.6%                                | 49.9%                        | 68.2%                         |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd)                              | 45.7%                                   | 38.9%                                | 34.6%                        | 85.9%                         |
| MessagePack (+zstd)                             | 47.1%                                   | 40.2%                                | 35.0%                        | 83.6%                         |
| Smile (+zstd)                                   | 48.0%                                   | 40.6%                                | 35.1%                        | 86.6%                         |
| Smile (+shared) (+zstd)                         | 48.0%                                   | 41.0%                                | 37.0%                        | 86.6%                         |
| frac json (+zstd)                               | 47.2%                                   | 40.5%                                | 36.8%                        | 80.8%                         |
| frac json (+global table) (+zstd)               | 25.1%                                   | 26.0%                                | 25.7%                        | 68.3%                         |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                | 14.3%                                   | 17.3%                                | 9.8%                         | 45.6%                         |
| MessagePack (+zstd +trained dict)               | 14.5%                                   | 17.3%                                | 10.0%                        | 45.2%                         |
| Smile (+zstd +trained dict)                     | 14.5%                                   | 17.0%                                | 9.8%                         | 45.0%                         |
| Smile (+shared) (+zstd +trained dict)           | 14.4%                                   | 17.5%                                | 9.9%                         | 44.7%                         |
| frac json (+zstd +trained dict)                 | 14.5%                                   | 17.9%                                | 9.8%                         | 45.4%                         |
| frac json (+global table) (+zstd +trained dict) | 13.2%                                   | 16.2%                                | 8.9%                         | 43.2%                         |

### Average encode time

|                                                 | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                      | 1.00x (2.4µs)                           | 1.00x (4.6µs)                        | 1.00x (5.0µs)                | 1.00x (250.0ns)               |
| MessagePack                                     | 0.69x (1.7µs)                           | 0.76x (3.5µs)                        | 0.70x (3.5µs)                | 0.67x (167.0ns)               |
| Smile                                           | 0.72x (1.8µs)                           | 0.84x (3.9µs)                        | 0.70x (3.5µs)                | 0.97x (242.0ns)               |
| Smile (+shared)                                 | 3.52x (8.6µs)                           | 3.55x (16.5µs)                       | 2.67x (13.3µs)               | 3.49x (872.0ns)               |
| frac json                                       | 3.61x (8.8µs)                           | 3.35x (15.5µs)                       | 2.24x (11.1µs)               | 2.58x (645.0ns)               |
| frac json (+global table)                       | 2.01x (4.9µs)                           | 2.50x (11.6µs)                       | 1.50x (7.5µs)                | 1.24x (310.0ns)               |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd)                              | 4.44x (10.8µs)                          | 4.34x (20.1µs)                       | 3.87x (19.2µs)               | 17.19x (4.3µs)                |
| MessagePack (+zstd)                             | 4.47x (10.9µs)                          | 4.11x (19.0µs)                       | 3.70x (18.4µs)               | 24.60x (6.2µs)                |
| Smile (+zstd)                                   | 4.57x (11.2µs)                          | 4.11x (19.0µs)                       | 3.65x (18.1µs)               | 19.42x (4.9µs)                |
| Smile (+shared) (+zstd)                         | 7.35x (18.0µs)                          | 6.94x (32.2µs)                       | 5.62x (27.9µs)               | 22.02x (5.5µs)                |
| frac json (+zstd)                               | 5.15x (12.6µs)                          | 5.31x (24.6µs)                       | 4.34x (21.5µs)               | 16.11x (4.0µs)                |
| frac json (+global table) (+zstd)               | 4.43x (10.8µs)                          | 4.70x (21.8µs)                       | 3.77x (18.7µs)               | 16.53x (4.1µs)                |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                | 8.69x (21.2µs)                          | 6.61x (30.6µs)                       | 5.73x (28.4µs)               | 68.04x (17.0µs)               |
| MessagePack (+zstd +trained dict)               | 8.81x (21.5µs)                          | 6.50x (30.2µs)                       | 5.32x (26.4µs)               | 67.91x (17.0µs)               |
| Smile (+zstd +trained dict)                     | 8.71x (21.3µs)                          | 6.34x (29.4µs)                       | 5.58x (27.7µs)               | 68.71x (17.2µs)               |
| Smile (+shared) (+zstd +trained dict)           | 11.65x (28.5µs)                         | 9.38x (43.5µs)                       | 7.53x (37.4µs)               | 71.36x (17.8µs)               |
| frac json (+zstd +trained dict)                 | 9.48x (23.2µs)                          | 7.94x (36.8µs)                       | 5.90x (29.3µs)               | 69.77x (17.4µs)               |
| frac json (+global table) (+zstd +trained dict) | 10.22x (25.0µs)                         | 8.28x (38.4µs)                       | 6.17x (30.6µs)               | 70.18x (17.5µs)               |

### Average decode time

|                                                 | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                      | 1.00x (4.7µs)                           | 1.00x (10.4µs)                       | 1.00x (12.3µs)               | 1.00x (512.0ns)               |
| MessagePack                                     | 1.14x (5.3µs)                           | 1.09x (11.3µs)                       | 1.05x (13.0µs)               | 0.87x (446.0ns)               |
| Smile                                           | 1.18x (5.5µs)                           | 1.13x (11.8µs)                       | 1.10x (13.5µs)               | 0.93x (476.0ns)               |
| Smile (+shared)                                 | 1.44x (6.7µs)                           | 1.23x (12.8µs)                       | 1.09x (13.4µs)               | 1.25x (639.0ns)               |
| frac json                                       | 2.60x (12.1µs)                          | 1.59x (16.5µs)                       | 1.18x (14.4µs)               | 2.34x (1.2µs)                 |
| frac json (+global table)                       | 1.15x (5.4µs)                           | 1.02x (10.6µs)                       | 0.70x (8.6µs)                | 0.77x (393.0ns)               |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd)                              | 1.78x (8.3µs)                           | 1.54x (16.1µs)                       | 1.45x (17.8µs)               | 4.82x (2.5µs)                 |
| MessagePack (+zstd)                             | 1.96x (9.1µs)                           | 1.59x (16.6µs)                       | 1.49x (18.3µs)               | 6.95x (3.6µs)                 |
| Smile (+zstd)                                   | 2.06x (9.6µs)                           | 1.65x (17.2µs)                       | 1.52x (18.7µs)               | 5.34x (2.7µs)                 |
| Smile (+shared) (+zstd)                         | 2.22x (10.4µs)                          | 1.74x (18.1µs)                       | 1.53x (18.8µs)               | 5.58x (2.9µs)                 |
| frac json (+zstd)                               | 1.80x (8.4µs)                           | 1.47x (15.3µs)                       | 1.13x (13.9µs)               | 4.51x (2.3µs)                 |
| frac json (+global table) (+zstd)               | 1.28x (6.0µs)                           | 1.34x (14.0µs)                       | 1.09x (13.4µs)               | 3.73x (1.9µs)                 |
|                                                 |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                | 2.17x (10.1µs)                          | 1.76x (18.3µs)                       | 1.56x (19.2µs)               | 9.79x (5.0µs)                 |
| MessagePack (+zstd +trained dict)               | 2.35x (11.0µs)                          | 1.83x (19.1µs)                       | 1.59x (19.5µs)               | 9.57x (4.9µs)                 |
| Smile (+zstd +trained dict)                     | 2.37x (11.1µs)                          | 1.84x (19.2µs)                       | 1.62x (19.9µs)               | 9.74x (5.0µs)                 |
| Smile (+shared) (+zstd +trained dict)           | 2.57x (12.0µs)                          | 1.96x (20.4µs)                       | 1.64x (20.1µs)               | 10.10x (5.2µs)                |
| frac json (+zstd +trained dict)                 | 2.11x (9.9µs)                           | 1.72x (17.8µs)                       | 1.20x (14.7µs)               | 9.45x (4.8µs)                 |
| frac json (+global table) (+zstd +trained dict) | 2.10x (9.8µs)                           | 1.68x (17.5µs)                       | 1.20x (14.7µs)               | 9.33x (4.8µs)                 |

### Takeaways

1. For large JSON files, there isn't much of a difference when using compression, between a custom JSON encoding and plain text.
2. For small files, regardless of compression, basic fractured json is roughly en par with other JSON encodings, sometimes a bit better, sometimes a bit worse.
3. For small files, regardless of compression, fractured json when used with an external keys table, outperforms other JSON encodings.
4. For small files, zstd compression with a pre trained dictionary, significantly outperforms all other options, in terms of storage size. However it comes at a encoding time cost (10x to 100x time increase).

In conclusion, the most optimal method depends on file size, as well as storage and time constraints. When in doubt, run benchmarks with your data.
