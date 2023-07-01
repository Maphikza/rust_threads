#[allow(unused)]
use std::{
 cell::{Cell, RefCell, UnsafeCell},
 collections::VecDeque,
 marker::PhantomData,
 mem::{ManuallyDrop, MaybeUninit},
 ops::{Deref, DerefMut},
 ptr::NonNull,
 rc::Rc,
 sync::{*, atomic::{*, Ordering::*}},
 thread::{self, Thread},
};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f_two);

    println!("This is the the main thread!");

    let answer = t1.join().unwrap() + t2.join().unwrap();

    println!("t1: {:?}", answer);
}

fn f() -> i32 {
    println!("hello from another thread.");
    let sum = 1+1;

    let id = thread::current().id();
    println!("this is my thread id: {:?}, with sum {}.", id, &sum);
    sum
}

fn f_two() -> i32 {
    println!("hello from another thread.");
    let sum = 1+5;

    let id = thread::current().id();
    println!("this is my thread id: {:?}, with sum {}.", id, &sum);
    sum
}
