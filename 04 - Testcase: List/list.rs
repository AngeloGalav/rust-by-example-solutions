use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?; // by now, you have noticed that sometimes we use print, and sometimes we use write. 
        // The reason for that being that write always write on whatever fmt is called on, while print only writes on 
        // the stdout

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.

            // Essentially, it unpacks the result if OK, otherwise it returns the error
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
            // original was: write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}