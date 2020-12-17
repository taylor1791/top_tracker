# Top 100 IPs

<!--
Imagine you want to track the IP address that are making the most requests to
your web service every day. Write a program that tracks these IP addresses in
memory and can return the top 100 most common IP addresses. Assume your web
service has thousands of unique high traffic IP addresses with a long tail of
millions of unique IP addresses that are lower traffic.

Your program must implement the following functions.

 * **request_handled(ip_address)**: Accepts a string containing an IP address e.g.
   145.87.2.109. This function will be called by the web service every time it
   handles a request, tens of millions of times per day. The calling code is
   outside the scope of this challenge.

 * **top100()**: Returns the top 100 IP addresses by the request count, with the
   highest traffic IP address first. This function must be fast enough to
   display on a dashboard.

 * **clear()**: Called at the start of each day to forget all IP addresses and
   tallies.
-->

## Quick Start

Just wanna see the code? Open [lib.rs](src/lib.rs) for the aforementioned
function implementations and [top_tracker.rs](src/top_tracker.rs) for the data
structure supporting the whole thing.

If you are here for a little more fun, you can checkout the [tests](tests) and
[benchmarks](benches/). They are as portable as Rust, but we recommend using a
64-bit machine and at least 2 gigabytes of available memory before attempting to
run them. Installing rust is left as an exercise to the reader. Then, run the
tests with `cargo test, or the benchmarks with `cargo bench`.

## Analysis

First, I concretized and codified the performance critera of `request_handled`
and `top100` in the [ip-counter tests](tests/ip_counter.rs). Then, I created
[benchmarks](benches/performance_criteria.rs) to estimate what kind of
algorithmic complexity met the required performance characteristics. With
`request_handled` requiring sub-logarithmic time (to the total number of IPs)
and `top_100` requiring sub-linear time (to the total number of IPs), one
implemention was obvious.

The implementation stores all the IPs in a HashMap and maintains a
"de-normalized" vector of the top IPs. Since counts only ever increment, a
single round of bubble sort is sufficent to keep it sorted.
