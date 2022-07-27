//! Custom Data Structures
//!
//! This module is mainly for learning Rust, by implementing standard
//! data structures.
//! Currently implemented are the:
//! - [`LinkedList`], a singly linked list.
//! - [`Stack`], a LIFO stack.
//! - [`PriorityQueue`], queue with in order insertion.
//!
//! [`LinkedList`]: ./linkedlist/struct.LinkedList.html
//! [`Stack`]: ./linkedlist/type.Stack.html
//! [`PriorityQueue`]: ./queues/struct.PriorityQueue.html

/// Module for the LinkedList.
///
/// LinkedList is singly linked, and has basic append/push/pop/peek/insert
/// capabilities. It can also be turned into an interator or be created from
/// an interator. Such a linked list is best used as a stack, where the only
/// interaction is done at the head of the list.
///
/// In this module is also an alias for a Stack, which is just a linked list but only uses pop and
/// push for interaction.
#[allow(dead_code)]
pub mod linkedlist;

/// Module for queue data structures
///
/// Still to implement
#[allow(dead_code)]
pub mod queues;
