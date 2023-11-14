To compile and run the benchmark:

```
$ cargo build --release
$ ./target/release/i139
```

Profiling the execution (e.g., `perf` on Linux) should reveal a hot loop where
most of the time is being spent on `x86-64`:

```
  0.02 │440:┌─→add        $0x20,%rcx
  0.02 │    │  cmp        %rdx,%rcx
       │    │↑ ja         108
  0.05 │44d:│  mov        0x10(%rsp),%rsi
 32.64 │    │  vpcmpeqb   (%rcx,%rsi,1),%ymm0,%ymm2
       │    │  mov        0x8(%rsp),%rsi
 30.66 │    │  vpcmpeqb   (%rcx,%rsi,1),%ymm1,%ymm3
  0.71 │    │  vpand      %ymm2,%ymm3,%ymm2
  3.69 │    │  vpmovmskb  %ymm2,%ebp
  0.20 │    ├──test       %ebp,%ebp
 28.57 │    └──je         440
```

This is where the code *should* be spending most of its time. But if you change
the dependency on `memchr` to be `=2.5.0`, then the hot loop should now look
like this:

```
  0.02 │280:┌─→add          $0x20,%rcx
  0.02 │    │  cmp          %rsi,%rcx
       │    │↑ ja           123
 32.63 │28d:│  vpcmpeqb     (%rcx,%rdx,1),%ymm0,%ymm2
  7.18 │    │  vpcmpeqb     (%rcx,%rdi,1),%ymm1,%ymm3
  1.20 │    │  vpand        %ymm2,%ymm3,%ymm2
 27.80 │    │  vpmovmskb    %ymm2,%r12d
  0.09 │    ├──test         %r12d,%r12d
 28.61 │    └──je           280
 ```

Notice that there are two fewer instructions here. Specifically, the two
`mov` instructions are now gone. This appears to be a result of some code
shuffling done between the 2.5.0 and 2.6.0 release. I can't figure out how
to get back to the code without the extra `mov` instructions.

Does it matter? [this issue][i139] seems to suggest it does, but the degree
to which it matters seems to depend on the CPU. In my case, I observe a ~6%
regression on the benchmark in this repository, but the issue reported sees a
much bigger regression of around ~30%. (I have an Intel CPU while the reporter
has an AMD CPU.)

[i139]: https://github.com/BurntSushi/memchr/issues/139
