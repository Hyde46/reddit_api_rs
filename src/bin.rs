extern crate rsreddit;

use rsreddit::client::Reddit;
use rsreddit::model::listing::Listing;
use rsreddit::model::listing::ListingCollection;

fn main() {
    // Get Comment Tree for a thread by its permalink
    let reddit = Reddit::default().build();
    let tree = reddit
        .thread_by_permalink(
            "/r/rust/comments/hwuvmf/swc_now_works_with_stable_rustc/",
        )
        .unwrap();
    print_comment_tree(&tree);
}

fn print_comment_tree(tree: &ListingCollection) {
    //print Thread Title
    println!(
        "Title: {}",
        tree.listings[0].data.children[0]
            .data
            .title
            .as_ref()
            .unwrap()
    );
    //Print comments recursivley
    travers_comment_tree(&tree.listings[1], 0);
}

fn travers_comment_tree(comment: &Listing, tab_count: usize) {
    comment.data.children.iter().for_each(|c| {
        //Generate right amount if identation
        let tabs = std::iter::repeat("   ").take(tab_count).collect::<String>();
        //Print comment
        println!("{}>{}", tabs, c.data.body.as_ref().unwrap());
        //Go a level deeper
        if let Some(replies) = &c.data.replies {
            travers_comment_tree(replies,tab_count + 1);
        }
    });
}
