use std::net::IpAddr;

pub mod top_tracker;

use top_tracker::TopTracker;

static TOP_COUNT: usize = 100;

/// Keeps track of the 100 IP addresses making the most requests.
///
/// `Top100Ips` will keep track of every IP making requests to your web service. It does so
/// efficiently and has fun while doing it. Simply call `request_handled`, just like this. It's so
/// easy that anyone can do it!
///
/// ```
/// use top_100_ips::Top100Ips;
///
/// let mut tracker = Top100Ips::new();
/// tracker.request_handled(String::from("192.168.1.1"));
/// ```
///
/// `Top100Ips` will keep track of all IP address _in memory_ and will not forget them until you
/// call [clear](#method.clear). If your users visit your site from 100s of millions of unique IPs,
/// you should not use this. In fact, I wouldn't use this at all because the author does not use it
/// in production.
///
/// When you are ready for the results simply call `top_100()`. Please see
/// [top_100](#method.top_100) for additional details.
pub struct Top100Ips {
    tracker: TopTracker<IpAddr>,
}

impl Top100Ips {
    pub fn new() -> Top100Ips {
        Top100Ips {
            tracker: TopTracker::new(TOP_COUNT),
        }
    }

    /// Record a request was handled from `ip`.
    pub fn request_handled(&mut self, ip: String) {
        // Did you want 192.168.01.1 to be handled differently than 192.168.1.1? FTFY. Okay, the
        // real reason I parse the IPs is to `impl Copy`. Rust's ownership model makes it "hard" to
        // do without.
        let ip: IpAddr = ip.parse().expect("Invalid IP Address");

        self.tracker.increment(ip);
    }

    /// The top 100 IPs by request count.
    ///
    /// In the case of a tie, the order of the IPs is undefined. This may lead to especially
    /// surprising behavior near the end of the list as there may be many IPs with the same count
    /// that are not included.
    pub fn top_100(&self) -> Vec<(String, u64)> {
        self.tracker
            .top()
            .iter()
            // No matter how many times I read "return the top 100 IP addresses by request count," I
            // don't know if the dashboard needs the IPs and the count or just the IPs. Since
            // clarifications would make the problem less fun, I am going to do what I would want
            // to see on the dashboard.
            .map(|(ip, count)| (format!("{}", ip), *count))
            .collect()
    }

    /// Forget all IPs the counts to begin anew.
    pub fn clear(&mut self) {
        self.tracker = TopTracker::new(TOP_COUNT);
    }
}
