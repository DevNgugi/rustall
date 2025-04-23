Cargo new <app name>
Cargo run/ build


*Concepts:*
---------
Ownership:
 - Each value has only a variable that is its owner
 - There can only be one owner at a time
 - The value is dropped when the owner is out of scope

Borrowing & References:
 - You create references from borrowing from the original owner (without taking ownership)
 - References can be Immutable/mutable
 - Use & 
Shadowing - 