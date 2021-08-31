/*

# Macro

What are Rust macros?
Rust has excellent support for macros.
Macros enable you to write code that writes other code, which is known as metaprogramming.

Macros provide functionality similar to functions but without the runtime cost.
There is some compile-time cost, however, since macros are expanded during compile time.

Rust macros are very different from macros in C.
Rust macros are applied to the token tree whereas C macros are text substitution.

# Keywords

item — an item, like a function, struct, module, etc.
block — a block (i.e. a block of statements and/or an expression, surrounded by braces)
stmt — a statement
pat — a pattern
expr — an expression
ty — a type
ident — an identifier
path — a path (e.g., foo, ::std::mem::replace, transmute::<_, int>, …)
meta — a meta item; the things that go inside #[...] and #![...] attributes
tt — a single token tree
vis — a possibly empty Visibility qualifier
$($) — repeated block
, — seperator
* — zero or more

*/

macro_rules! add_as{
    (
  // repeated block
  $($a:expr)
    // seperator
   ,
    // zero or more
   *
   )=>{
       {
        // to handle the case without any arguments
        0
        // block to be repeated
        $(+$a)*
     }
    }
}

macro_rules! add{
    // first arm in case of single argument and last remaining variable/number
    ($a:expr)=>{
        $a
    };
    // second arm in case of two arument are passed and stop recursion in case of odd number ofarguments
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };
    // add the number and the result of remaining arguments
    ($a:expr,$($b:tt)*)=>{
        {
            $a+add!($($b)*)
        }
    }
}

macro_rules! ok_or_return{
    // match something(q,r,t,6,7,8) etc
    // compiler extracts function name and arguments. It injects the values in respective varibles.
    ($a:ident($($b:tt)*))=>{
        {
            match $a($($b)*) {
                Ok(value)=>value,
                Err(err)=>{
                    return Err(err);
                }
            }
        }
    };
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

fn main() -> Result<(), String> {
    println!("{}", add_as!(1, 2, 3, 4)); // => println!("{}",{0+1+2+3+4}) = 10
    println!("{}", add!(1, 2, 3, 4)); // 10
    ok_or_return!(some_work(1, 3));
    ok_or_return!(some_work(1, 0));
    Ok(())
}
