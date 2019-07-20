Im trying to follow Alexis Beingessner's  : Learning Rust With Entirely Too Many Linked Lists

https://rust-unofficial.github.io/too-many-lists/fifth-layout.html

This is the doubly linked list. Trying to apply all the principles I learned.

The tutorial was great, but it wasn't enough for me. I also referred to Programming Rust , especially the raw pointers stuff to make progress.


Some notes is myself, although may help another reader as well. 

The *l is dereferencing the Box (like you would do in C or C++ and has nothing to do with pointer syntax). I never realised this initially and struggled here. The OReilly book helped clear that.

            let raw_tail: *mut _ = &mut * l;

Why the double star here line 61 ?  I think it's because when we say :  
     Some (ref mut boxed_link) => 
     on lin 58, we are adding 1 more indirection that has to be stripped away.

            let raw_prev_head: * mut Link = &mut ** boxed_link;
