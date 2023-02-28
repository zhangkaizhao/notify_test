# notify test

Tested environment:

```shell
$ uname -mrs
Darwin 21.6.0 x86_64
$ rustc --version
rustc 1.67.1 (d5a82bbd2 2023-02-07)
```

Build and start it in a shell:

```shell
$ cargo build
$ ./target/debug/notify_test
```

Do some file operations one by one in another shell under the same directory:

```
$ touch hello.txt
$ echo "abc" >> hello.txt
$ echo "abc" >> hello.txt
$ rm hello.txt
```

Ctrl-C in the first shell and get all the output:

```text
start chating in background...
start watching...
I am chating!
I am chating!
event: Event { kind: Create(File), paths: ["/Users/kaizhao/lab/rust/notify_test/hello.txt"], attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
I am smilling.
I am crying.
I am chating!
event: Event { kind: Modify(Data(Content)), paths: ["/Users/kaizhao/lab/rust/notify_test/hello.txt"], attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
I am smilling.
I am crying.
event: Event { kind: Modify(Data(Content)), paths: ["/Users/kaizhao/lab/rust/notify_test/hello.txt"], attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
I am smilling.
I am crying.
I am chating!
event: Event { kind: Remove(File), paths: ["/Users/kaizhao/lab/rust/notify_test/hello.txt"], attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
I am smilling.
I am crying.
event: Event { kind: Modify(Data(Content)), paths: ["/Users/kaizhao/lab/rust/notify_test/hello.txt"], attr:tracker: None, attr:flag: None, attr:info: None, attr:source: None }
I am smilling.
I am crying.
I am chating!
^Cstopped.
```

Observation: There is still a `Modify(Data(Content))` event received after the `Remove(File)` event received.

Conclusion: The order in which notification events are received is not consistent with the order of file operations.
