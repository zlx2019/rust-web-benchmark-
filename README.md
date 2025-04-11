# Rust web framework benchmark

## Framework
- Axum
- Actix
- Rocket
- Wrap

## Axum
- 第一次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   437.73us  583.08us  21.79ms   95.36%
    Req/Sec    21.82k     6.98k  138.66k    81.54%
  2176561 requests in 10.10s, 267.77MB read
Requests/sec: 215436.63
Transfer/sec:     26.50MB
```

- 第二次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   410.59us  388.88us  13.18ms   93.17%
    Req/Sec    21.83k     6.63k  132.35k    84.73%
  2177792 requests in 10.10s, 267.92MB read
Requests/sec: 215576.50
Transfer/sec:     26.52MB
```

- 第三次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   417.15us  431.01us  20.23ms   93.56%
    Req/Sec    21.84k     6.75k  122.19k    82.24%
  2177972 requests in 10.10s, 267.94MB read
Requests/sec: 215654.14
Transfer/sec:     26.53MB
```

## Actix-Web
- 第一次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     0.88ms    3.22ms 100.41ms   96.46%
    Req/Sec    22.51k     8.04k   76.78k    69.87%
  2247267 requests in 10.10s, 278.61MB read
Requests/sec: 222571.34
Transfer/sec:     27.59MB
```

- 第二次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   703.01us    1.82ms  55.46ms   95.29%
    Req/Sec    22.53k     7.96k   84.31k    71.90%
  2247067 requests in 10.08s, 278.59MB read
Requests/sec: 222859.33
Transfer/sec:     27.63MB
```

- 第三次
```text
Running 10s test @ http://127.0.0.1:8080/index
  10 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   665.78us    1.71ms  52.68ms   95.82%
    Req/Sec    22.55k     7.97k  134.42k    73.50%
  2248712 requests in 10.09s, 278.79MB read
Requests/sec: 222875.09
Transfer/sec:     27.63MB
```



wrk 压测参数

```text
Running 30s test @ http://www.bing.com （压测时间30s）
  8 threads and 200 connections （共8个测试线程，200个连接）
  Thread Stats   Avg      Stdev     Max   +/- Stdev
              （平均值） （标准差）（最大值）（正负一个标准差所占比例）
    Latency    46.67ms  215.38ms   1.67s    95.59%
    （延迟）
    Req/Sec     7.91k     1.15k   10.26k    70.77%
    （处理中的请求数）
  Latency Distribution （延迟分布）
     50%    2.93ms
     75%    3.78ms
     90%    4.73ms
     99%    1.35s （99分位的延迟）
  1790465 requests in 30.01s, 684.08MB read （30.01秒内共处理完成了1790465个请求，读取了684.08MB数据）
Requests/sec:  59658.29 （平均每秒处理完成59658.29个请求）
Transfer/sec:     22.79MB （平均每秒读取数据22.79MB）
```