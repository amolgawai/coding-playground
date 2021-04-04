
fn main() {
    // Every value must have one and only one owner

    // copy and move concepts
    copy_and_move();
    borrow_concepts();
    slices();

}

fn copy_and_move() {

    // Values on stack can't transfer ownership, they are copied instead

    let a_str = "I am a string literal";
    let b_str = a_str; // Copy the string literal and assign owner

    println!("Copied stack string literals -> {}, {}", a_str, b_str);

    let a_heap_str1 = String::from("I am a heap string");
    println!("String -> {}", a_heap_str1);
    let b_heap_str1 = a_heap_str1; // ownership transferred (known as move)
    // println!("Compiler error -> {}", a_heap_str1);
    println!("Moved string -> {}", b_heap_str1);

    let a_heap_str2 = String::from("Heap string in a function");
    println!("Heap string created in one function -> {}", a_heap_str2);
    move_ownership(a_heap_str2); // string moved to function
    // can't use the heap as ownership moved to function
    // println!("Compiler error - {}", a_heap_str2);

    // cloning provides copy of heap entities
    let a_heap_str3 = String::from("I can be cloned");
    let b_heap_str3 = a_heap_str3.clone();
    println!("Original \"{}\" and cloned \"{}\"", a_heap_str3, b_heap_str3);
}

fn move_ownership(new_owner: String) {
    println!("Ownership of string transferred to function -> {}", new_owner);
}

fn borrow_concepts() {
    // Borrowing of heap items don't transfer ownership
    // let mut a_heap_str1 = ... would also work but emmit a warning
    let a_heap_str1 = String::from("For borrowing");
    // multiple read borrows possible
    let b_heap_str1 = &a_heap_str1;
    println!("Original \"{}\" and read only borrowed \"{}\"", a_heap_str1, b_heap_str1);
    read_only_borrow(&a_heap_str1);

    // Only one writable borrow possible
    let mut a_heap_str2 = String::from("mutable string");
    writable_borrow(&mut a_heap_str2);
    // This would generate compiler error
    // let c_heap_str2 = &a_heap_str2;
    // let b_heap_str2 = &mut a_heap_str2;
    // error[E0502]: cannot borrow `a_heap_str2` as mutable because it is also borrowed as immutable
    // println!("compile error {}, {}", c_heap_str2, b_heap_str2);
}

fn read_only_borrow(borrowed: &String) {
    println!("Read Only Borrowed in another function -> {}", borrowed);
}

fn writable_borrow(borrowed: &mut String) {
    borrowed.push_str(", borrow modification in a function");
    println!("{}", borrowed);
}

fn slices() {
    // Slices are always references
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slice 1 - {}, Slice 2 - {} of string - {}", hello, world, s);

    println!("First word slice - {}", first_word(&s));

    // other slices e.g. array slices
    let a_array = [0, 1, 2, 3, 4, 5];

    let a_slice = &a_array[..2];
    let b_slice = &a_array[2..];
    let c_slice = &a_array[..];

    println!("array slices - {:?}, {:?} and {:?}", a_slice, b_slice, c_slice);

}

fn first_word(sentence: &String) -> &str {

    let bytes = sentence.as_bytes();

    for (indx, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &sentence[0..indx];
        }
    }

    &sentence[..]
}
