extern crate rsreddit;

use rsreddit::client::Reddit;

fn main() {
    // Get Thread Tree for a thread by its permalink
    let reddit = Reddit::default().build();
    reddit.thread_by_permalink(
        "/r/rust/comments/hv78hc/show_rrust_a_tiny_unix_shell_from_c_to_rust_from/",
    );
}
