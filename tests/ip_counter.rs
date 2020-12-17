#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{Arbitrary, Gen};
use std::fmt::Display;
use std::time::{Duration, Instant};
use top_100_ips::Top100Ips;

#[derive(Debug, Clone)]
struct IpAddress {
    octet1: u8,
    octet2: u8,
    octet3: u8,
    octet4: u8,
}

impl Display for IpAddress {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}.{}.{}.{}",
            self.octet1, self.octet2, self.octet3, self.octet4,
        )
    }
}

impl Arbitrary for IpAddress {
    // This implementation may generate invalid IP addresses, e.g. 0.0.0.0, but that is not reason
    // for concern. If you don't believe me, run these tests.
    fn arbitrary<G: Gen>(g: &mut G) -> IpAddress {
        IpAddress {
            octet1: u8::arbitrary(g),
            octet2: u8::arbitrary(g),
            octet3: u8::arbitrary(g),
            octet4: u8::arbitrary(g),
        }
    }
}

#[test]
fn repeated_ips_bubble_to_top() {
    let mut tracker = Top100Ips::new();

    // This loop makes 192.168.1.100 occur 100 times, 192.168.1.99 occur 99 times, etc. However,
    // the tracker causes "maximum" bubbling work since iteration 1 will have 192.168.1.1 to
    // 192.168.1.100 in order. In iteration i, 192.168.1.i will be at the front.
    for i in 1..=100 {
        for j in i..=100 {
            let ip = format!("192.168.1.{}", j);
            tracker.request_handled(ip);
        }
    }

    let top = tracker.top_100();
    assert_eq!(top[0], (String::from("192.168.1.100"), 100));
    assert_eq!(top[100 - 35], (String::from("192.168.1.35"), 35)); //
    assert_eq!(top[99], (String::from("192.168.1.1"), 1));
}

#[quickcheck]
fn idempotent_between_clears(ips: Vec<IpAddress>) -> bool {
    let mut tracker = Top100Ips::new();
    ips.clone()
        .into_iter()
        .for_each(|ip| tracker.request_handled(ip.to_string()));

    let top1 = tracker.top_100();

    tracker.clear();
    ips.into_iter()
        .for_each(|ip| tracker.request_handled(ip.to_string()));

    let top2 = tracker.top_100();

    top1 == top2
}

// When designing a service, the most important traffic rate is peak second volume. Unfortunately,
// "tens of millions of requests per day" tells us very little about the peak second volume. If the
// distribution was uniform, it would mean 100s of requests per second. If the traffic were
// adversarial, tens of millions of requests would occur per second. Second, most services are
// designed for growth based on the expected lifetime of the service. One could make an accurate
// guess for both values with more information, but my intuition unjustifiably expects to see 5
// times the uniform traffic during peak hours and to scale for 10x growth. If `request_handled` is
// going to be called 5000 times per second, a single invocation should run in 200μs when there are
// 100 million unique IPs.
#[test]
fn can_handle_peak_traffic() {
    let mut tracker = generate_huge_tracker();
    let mut gen = quickcheck::StdThreadGen::new(100_000_000);
    let ip: IpAddress = Arbitrary::arbitrary(&mut gen);
    let ip_string = format!("{}", ip);

    let before = Instant::now();
    tracker.request_handled(ip_string);
    let after = Instant::now();

    assert!(after - before < Duration::from_micros(200));
}

// The only performance requirement of `top_100` is "fast". Since this is a web service, the user
// can expect network latency. Network latency is usually measured in milliseconds, so 1ms would be
// insanely fast and certainly not reasonable for real web service.
#[test]
fn top_100_is_fast_enough() {
    let tracker = generate_huge_tracker();

    let before = Instant::now();
    tracker.top_100();
    let after = Instant::now();

    assert!(after - before < Duration::from_millis(1));
}

fn generate_huge_tracker() -> Top100Ips {
    let mut gen = quickcheck::StdThreadGen::new(1_000_000); // "size" of each IP octet
    let mut tracker = Top100Ips::new();

    for _ in 0..100_000_000 {
        // This generates "uniform" IPs where each octet is limited by `gen`. This is not the
        // "assumed" distribution from the problem description. It should provide a pretty reliable
        // estimate nevertheless.
        let ip: IpAddress = Arbitrary::arbitrary(&mut gen);
        tracker.request_handled(format!("{}", ip));
    }

    tracker
}
