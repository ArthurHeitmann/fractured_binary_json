# Fractured Binary JSON

A binary JSON encoding optimized for small storage size.

When the schema of JSON data is known ahead of time, storage size can be further optimized by storing object key names in a separate global keys table.

Format specification can be found [here](./format_specification.md).

## Benchmarks

Comparison of different JSON encoding formats in combination with compression.

Tables generated with [benchmark](./frac_json_rust/benchmark/) Rust sub-project.

Zstd used with compression level 3 and pre trained dictionary is 100 KB.

Relative encode and decode performance will differ in different languages.

Takeaways at the end.

### Storage size for small JSON files (< 5 KB):

|                                       | MediaContent.json | reddit_comments_1.json | reddit_posts_1.json | TwitterTimeline.json | CouchDB4k.json  |
|---------------------------------------|-------------------|------------------------|---------------------|----------------------|-----------------|
| plain text                            | 100.0% (485.0 B)  | 100.0% (1.8 KB)        | 100.0% (3.6 KB)     | 100.0% (2.0 KB)      | 100.0% (3.8 KB) |
| MessagePack                           | 80.4%             | 80.2%                  | 81.7%               | 83.4%                | 80.1%           |
| Smile                                 | 82.1%             | 80.9%                  | 82.6%               | 84.1%                | 84.9%           |
| Smile (+shared)                       | 66.4%             | 80.9%                  | 79.5%               | 83.0%                | 71.9%           |
| fracture json                         | 71.1%             | 80.2%                  | 79.3%               | 82.0%                | 75.0%           |
| fracture json (+global table)         | 55.3%             | 33.1%                  | 42.6%               | 39.7%                | 72.8%           |
|                                       |                   |                        |                     |                      |                 |
| plain text (+zstd)                    | 56.5%             | 48.4%                  | 41.6%               | 44.8%                | 41.0%           |
| MessagePack (+zstd)                   | 52.6%             | 49.2%                  | 43.2%               | 46.5%                | 43.4%           |
| Smile (+zstd)                         | 52.2%             | 50.2%                  | 43.5%               | 47.0%                | 43.5%           |
| Smile (+shared) (+zstd)               | 52.6%             | 50.2%                  | 43.6%               | 47.0%                | 43.8%           |
| fracture json (+zstd)                 | 51.8%             | 49.3%                  | 43.3%               | 46.0%                | 42.2%           |
| fracture json (+global table) (+zstd) | 38.6%             | 28.2%                  | 26.2%               | 27.5%                | 40.7%           |

### Storage size for big JSON files (> 100 KB):

|                                       | reddit_comments_100.json | reddit_posts_100.json | twitter.json      | citm_catalog.json | canada.json     | jeopardy.json    |
|---------------------------------------|--------------------------|-----------------------|-------------------|-------------------|-----------------|------------------|
| plain text                            | 100.0% (205.0 KB)        | 100.0% (381.0 KB)     | 100.0% (456.0 KB) | 100.0% (488.6 KB) | 100.0% (2.0 MB) | 100.0% (50.1 MB) |
| MessagePack                           | 81.8%                    | 82.9%                 | 86.0%             | 68.5%             | 51.2%           | 87.9%            |
| Smile                                 | 82.2%                    | 83.5%                 | 86.4%             | 75.4%             | 64.7%           | 88.2%            |
| Smile (+shared)                       | 41.2%                    | 46.0%                 | 42.3%             | 37.8%             | 64.7%           | 51.2%            |
| fracture json                         | 40.8%                    | 46.9%                 | 50.1%             | 37.3%             | 29.6%           | 66.6%            |
| fracture json (+global table)         | 40.4%                    | 46.2%                 | 49.8%             | 27.5%             | 29.6%           | 66.6%            |
|                                       |                          |                       |                   |                   |                 |                  |
| plain text (+zstd)                    | 13.0%                    | 14.9%                 | 8.8%              | 2.5%              | 27.4%           | 22.8%            |
| MessagePack (+zstd)                   | 13.2%                    | 15.1%                 | 9.2%              | 2.6%              | 22.2%           | 23.7%            |
| Smile (+zstd)                         | 13.1%                    | 14.8%                 | 9.0%              | 2.6%              | 20.9%           | 23.1%            |
| Smile (+shared) (+zstd)               | 12.4%                    | 14.4%                 | 8.5%              | 2.3%              | 20.9%           | 21.8%            |
| fracture json (+zstd)                 | 12.5%                    | 14.4%                 | 8.6%              | 2.4%              | 17.4%           | 22.4%            |
| fracture json (+global table) (+zstd) | 12.2%                    | 14.1%                 | 8.4%              | 2.2%              | 17.4%           | 22.4%            |

### Average storage size across many small objects

|                                                     | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-----------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                          | 100.0% (1.7 KB)                         | 100.0% (3.9 KB)                      | 100.0% (4.6 KB)              | 100.0% (241.1 B)              |
| MessagePack                                         | 78.8%                                   | 82.7%                                | 86.0%                        | 88.3%                         |
| Smile                                               | 79.5%                                   | 83.5%                                | 86.5%                        | 90.3%                         |
| Smile (+shared)                                     | 79.1%                                   | 80.4%                                | 66.9%                        | 90.3%                         |
| fracture json                                       | 78.8%                                   | 80.7%                                | 69.4%                        | 89.3%                         |
| fracture json (+global table)                       | 30.1%                                   | 47.5%                                | 49.9%                        | 68.2%                         |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd)                                  | 45.7%                                   | 38.9%                                | 34.6%                        | 85.9%                         |
| MessagePack (+zstd)                                 | 47.1%                                   | 40.2%                                | 35.0%                        | 83.6%                         |
| Smile (+zstd)                                       | 48.0%                                   | 40.6%                                | 35.1%                        | 86.6%                         |
| Smile (+shared) (+zstd)                             | 48.0%                                   | 41.0%                                | 37.0%                        | 86.6%                         |
| fracture json (+zstd)                               | 47.2%                                   | 40.5%                                | 36.8%                        | 80.8%                         |
| fracture json (+global table) (+zstd)               | 25.1%                                   | 25.9%                                | 25.7%                        | 68.3%                         |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                    | 13.3%                                   | 15.8%                                | 11.8%                        | 41.1%                         |
| MessagePack (+zstd +trained dict)                   | 13.5%                                   | 15.6%                                | 10.4%                        | 40.8%                         |
| Smile (+zstd +trained dict)                         | 13.5%                                   | 15.6%                                | 10.5%                        | 40.5%                         |
| Smile (+shared) (+zstd +trained dict)               | 13.2%                                   | 15.8%                                | 9.5%                         | 40.5%                         |
| fracture json (+zstd +trained dict)                 | 13.6%                                   | 16.1%                                | 9.8%                         | 40.7%                         |
| fracture json (+global table) (+zstd +trained dict) | 12.2%                                   | 14.7%                                | 6.9%                         | 38.9%                         |

### Average encode time

|                                                     | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-----------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                          | 1.00x (2.0µs)                           | 1.00x (4.4µs)                        | 1.00x (4.8µs)                | 1.00x (258.0ns)               |
| MessagePack                                         | 0.76x (1.5µs)                           | 0.75x (3.3µs)                        | 0.66x (3.2µs)                | 0.65x (168.0ns)               |
| Smile                                               | 0.73x (1.5µs)                           | 0.74x (3.3µs)                        | 0.67x (3.3µs)                | 0.91x (234.0ns)               |
| Smile (+shared)                                     | 4.13x (8.4µs)                           | 3.65x (16.2µs)                       | 2.60x (12.5µs)               | 3.11x (803.0ns)               |
| fracture json                                       | 1.64x (3.3µs)                           | 2.09x (9.2µs)                        | 1.31x (6.3µs)                | 0.88x (226.0ns)               |
| fracture json (+global table)                       | 1.99x (4.0µs)                           | 2.46x (10.9µs)                       | 1.57x (7.6µs)                | 1.15x (296.0ns)               |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd)                                  | 5.24x (10.7µs)                          | 4.30x (19.1µs)                       | 3.81x (18.4µs)               | 16.31x (4.2µs)                |
| MessagePack (+zstd)                                 | 5.19x (10.5µs)                          | 4.11x (18.2µs)                       | 3.56x (17.2µs)               | 16.85x (4.3µs)                |
| Smile (+zstd)                                       | 5.21x (10.6µs)                          | 4.10x (18.2µs)                       | 3.49x (16.8µs)               | 18.28x (4.7µs)                |
| Smile (+shared) (+zstd)                             | 8.66x (17.6µs)                          | 7.08x (31.3µs)                       | 5.69x (27.5µs)               | 20.88x (5.4µs)                |
| fracture json (+zstd)                               | 6.11x (12.4µs)                          | 5.46x (24.2µs)                       | 4.25x (20.5µs)               | 15.17x (3.9µs)                |
| fracture json (+global table) (+zstd)               | 5.15x (10.5µs)                          | 4.80x (21.2µs)                       | 3.74x (18.1µs)               | 15.52x (4.0µs)                |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                    | 93.81x (190.7µs)                        | 50.19x (222.2µs)                     | 30.21x (145.8µs)             | 923.53x (238.3µs)             |
| MessagePack (+zstd +trained dict)                   | 96.79x (196.8µs)                        | 50.08x (221.7µs)                     | 40.50x (195.5µs)             | 934.59x (241.1µs)             |
| Smile (+zstd +trained dict)                         | 98.49x (200.2µs)                        | 50.13x (221.9µs)                     | 40.50x (195.5µs)             | 930.11x (240.0µs)             |
| Smile (+shared) (+zstd +trained dict)               | 102.84x (209.1µs)                       | 53.33x (236.1µs)                     | 37.45x (180.7µs)             | 935.39x (241.3µs)             |
| fracture json (+zstd +trained dict)                 | 101.46x (206.3µs)                       | 52.70x (233.3µs)                     | 40.68x (196.3µs)             | 933.64x (240.9µs)             |
| fracture json (+global table) (+zstd +trained dict) | 120.70x (245.4µs)                       | 56.44x (249.9µs)                     | 32.90x (158.8µs)             | 962.09x (248.2µs)             |

### Average decode time

|                                                     | reddit_comments_10k.json (16.7MB / 10k) | reddit_posts_10k.json (37.7MB / 10k) | twitter.json (455.5KB / 100) | jeopardy.json (49.9MB / 216k) |
|-----------------------------------------------------|-----------------------------------------|--------------------------------------|------------------------------|-------------------------------|
| plain text                                          | 1.00x (4.6µs)                           | 1.00x (10.2µs)                       | 1.00x (12.4µs)               | 1.00x (504.0ns)               |
| MessagePack                                         | 1.18x (5.5µs)                           | 1.10x (11.3µs)                       | 1.04x (12.9µs)               | 0.87x (439.0ns)               |
| Smile                                               | 1.13x (5.3µs)                           | 1.11x (11.4µs)                       | 1.04x (12.9µs)               | 0.90x (453.0ns)               |
| Smile (+shared)                                     | 1.37x (6.4µs)                           | 1.21x (12.3µs)                       | 1.06x (13.1µs)               | 1.17x (590.0ns)               |
| fracture json                                       | 0.94x (4.3µs)                           | 0.93x (9.6µs)                        | 0.69x (8.5µs)                | 0.75x (380.0ns)               |
| fracture json (+global table)                       | 0.95x (4.4µs)                           | 0.98x (10.0µs)                       | 0.68x (8.4µs)                | 0.75x (378.0ns)               |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd)                                  | 1.78x (8.3µs)                           | 1.52x (15.6µs)                       | 1.42x (17.5µs)               | 5.00x (2.5µs)                 |
| MessagePack (+zstd)                                 | 1.98x (9.2µs)                           | 1.61x (16.5µs)                       | 1.49x (18.5µs)               | 5.27x (2.7µs)                 |
| Smile (+zstd)                                       | 2.00x (9.3µs)                           | 1.63x (16.6µs)                       | 1.47x (18.2µs)               | 5.40x (2.7µs)                 |
| Smile (+shared) (+zstd)                             | 2.19x (10.2µs)                          | 1.73x (17.7µs)                       | 1.48x (18.4µs)               | 5.77x (2.9µs)                 |
| fracture json (+zstd)                               | 1.79x (8.3µs)                           | 1.48x (15.1µs)                       | 1.11x (13.7µs)               | 4.59x (2.3µs)                 |
| fracture json (+global table) (+zstd)               | 1.28x (6.0µs)                           | 1.36x (14.0µs)                       | 1.05x (13.0µs)               | 3.81x (1.9µs)                 |
|                                                     |                                         |                                      |                              |                               |
| plain text (+zstd +trained dict)                    | 2.65x (12.3µs)                          | 1.92x (19.7µs)                       | 1.69x (20.9µs)               | 13.19x (6.6µs)                |
| MessagePack (+zstd +trained dict)                   | 2.78x (12.9µs)                          | 1.98x (20.3µs)                       | 1.70x (21.1µs)               | 12.68x (6.4µs)                |
| Smile (+zstd +trained dict)                         | 2.77x (12.8µs)                          | 1.99x (20.4µs)                       | 1.74x (21.6µs)               | 12.80x (6.4µs)                |
| Smile (+shared) (+zstd +trained dict)               | 2.95x (13.7µs)                          | 2.09x (21.4µs)                       | 1.72x (21.2µs)               | 13.21x (6.7µs)                |
| fracture json (+zstd +trained dict)                 | 2.65x (12.3µs)                          | 1.88x (19.3µs)                       | 1.42x (17.6µs)               | 12.46x (6.3µs)                |
| fracture json (+global table) (+zstd +trained dict) | 2.46x (11.4µs)                          | 1.86x (19.0µs)                       | 1.31x (16.3µs)               | 12.42x (6.3µs)                |

### Takeaways

1. For large JSON files, there isn't much of a difference when using compression, between a custom JSON encoding and plain text.
2. For small files, regardless of compression, basic fractured json is roughly en par with other JSON encodings, sometimes a bit better, sometimes a bit worse.
3. For small files, regardless of compression, fractured json when used with an external keys table, significantly outperforms other JSON encodings.
4. For small files, zstd compression with a pre trained dictionary, significantly outperforms all other options, in terms of storage size. However it comes at a very high encoding time cost (30x to 1000x time increase).

In conclusion, the most optimal method depends on file size, as well as storage and time constraints. When in doubt, run benchmarks with your data.
