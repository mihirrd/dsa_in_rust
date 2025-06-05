/**
Determine if two stings are anagrams of each other.

The below appraoch uses two hashmaps.
 **/


fn are_anagrams(s1: String, s2: String) -> bool {

    if s1.len() != s2.len(){
        return false;
    }

    let mut h1 = HashMap::new();
    let mut h2 = HashMap::new();
    for c in s1.chars() {
        *h1.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
         *h2.entry(c).or_insert(0) += 1;
    }

    return h1 == h2;

}
