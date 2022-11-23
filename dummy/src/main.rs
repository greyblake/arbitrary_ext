use arbitrary::{Arbitrary, Unstructured};
use arbitrary_ext::{
    arbitrary_btree_set,
    arbitrary_hash_set,
    arbitrary_option,
    arbitrary_vec,
    arbitrary_hash_map,
    arbitrary_btree_map,
    arbitrary_vec_deque,
    arbitrary_linked_list,
    arbitrary_binary_heap,
};

use std::collections::{BTreeSet, HashSet, HashMap, BTreeMap, VecDeque, LinkedList, BinaryHeap};


// Some foreign type, that by some reason does not implement Arbitrary trait.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Number(u32);

// Our custom function to generate arbitrary Number out of Unstructured.
fn arbitrary_number(u: &mut Unstructured) -> arbitrary::Result<Number> {
    let value = u.int_in_range(0..=1000)?;
    Ok(Number(value))
}


#[derive(Debug, Arbitrary, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Alice,
    Bob,
    Julia,
    Kate,
}

fn arbitrary_player(u: &mut Unstructured) -> arbitrary::Result<Player> {
    Player::arbitrary(u)
}

#[derive(Debug, Arbitrary)]
struct Example {
    #[arbitrary(with = arbitrary_number)]
    number: Number,

    #[arbitrary(with = arbitrary_option(arbitrary_number))]
    option: Option<Number>,

    #[arbitrary(with = arbitrary_vec(arbitrary_number))]
    vec: Vec<Number>,

    #[arbitrary(with = arbitrary_vec_deque(arbitrary_number))]
    vec_deque: VecDeque<Number>,

    #[arbitrary(with = arbitrary_linked_list(arbitrary_number))]
    linked_list: LinkedList<Number>,

    #[arbitrary(with = arbitrary_btree_set(arbitrary_option(arbitrary_number)))]
    btree_set: BTreeSet<Option<Number>>,

    #[arbitrary(with = arbitrary_hash_set(arbitrary_option(arbitrary_number)))]
    hash_set: HashSet<Option<Number>>,

    #[arbitrary(
        with = arbitrary_hash_map(
            arbitrary_player,
            arbitrary_vec(arbitrary_number)
        )
    )]
    hash_map: HashMap<Player, Vec<Number>>,

    #[arbitrary(
        with = arbitrary_btree_map(
            arbitrary_player,
            arbitrary_btree_set(arbitrary_number)
        )
    )]
    btree_map: BTreeMap<Player, BTreeSet<Number>>,

    #[arbitrary(with = arbitrary_binary_heap(arbitrary_number))]
    binary_heap: BinaryHeap<Number>,
}

fn main() {
    let example = gen_arbitrary();
    println!("{example:#?}");
}

fn gen_arbitrary() -> Example {
    use rand::RngCore;
    //let mut bytes = [0u8; 65536];
    let mut bytes = [0u8; 65_536];
    rand::thread_rng().fill_bytes(&mut bytes);
    let mut u = Unstructured::new(&bytes);

    Example::arbitrary(&mut u).unwrap()

}

fn run_in_loop() {
    let mut count = 0;
    loop {
        gen_arbitrary();
        count += 1;
        if count % 100_000 == 0 {
            println!("Count = {count}");
        }
    }
}
