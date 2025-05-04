# Quick and Dirty Plot tool

Plot in terminal tool

## Quick start

1. get some data in a CSV file

```example.csv
,I,L,S
-5,-5,NaN,0.9589242746631385
-4,-4,NaN,0.7568024953079282
-3,-3,NaN,-0.1411200080598672
-2,-2,NaN,-0.9092974268256817
-1,-1,NaN,-0.8414709848078965
0,0,NaN,0.0
1,1,0.0,0.8414709848078965
2,2,1.0,0.9092974268256817
3,3,1.584962500721156,0.1411200080598672
4,4,2.0,-0.7568024953079282
5,5,2.321928094887362,-0.9589242746631385
```

The first line correspond to dataset labels. Each dataset will be represented
by the first letter of their label.
The first column correspond to x, the following one to y (one olumn by dataset)

2. draw a simple plot

```bash
cargo run -- example.csv
```

3. explore the command line

```bash
cargo run -- --help
```
