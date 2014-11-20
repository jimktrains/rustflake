grustflake
=========

A Rust implementation of [Twitter's Snowflake](https://github.com/twitter/snowflake/).

The basic rundown is that it creats 64-bit IDs that are partially ordered
by time and also unique over time and space. There are protections to make sure the sequence counter doesn't rollover twice in the same ms. Also, it attempts to ensure time doesn't go backwards (NTP update or soemthing).

One difference between rustflake and snowflake is that snowflake has a modified epoch.  I need to reach into the code to figure out what theirs is and add it to mine.

Layout:

|  Size  | Description |
| ------ | ----------- |
| 41     | ms Timestamp|
| 10     | machine id  |
| 12     | sequence    |
