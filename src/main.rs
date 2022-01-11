// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
//use std::mem;


pub struct ListNode {
    pub val: i64,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i64) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn split_list(l1:Option<Box<ListNode>>, mut reconstructed_number: Vec<i64>) -> Vec<i64> {

        let l1unwrap = l1.as_ref().unwrap();
        println!("{:?}", l1unwrap.val);
        reconstructed_number.push(*&l1unwrap.val);
//        println!("{:?}", l1unwrap.next);
        if l1unwrap.next == None {return reconstructed_number;}
        let nextnode= l1unwrap.next.to_owned();
        ListNode::split_list(nextnode, reconstructed_number)
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    println!("Got here");
    return l1;
}

pub fn combine_vector(vec: Vec<i64>) -> i64 {
    let mut no1:i64 = 0;
    let mut mult:i64 = 1;
    for num in vec {
        no1 = no1 + ( num * mult);
        mult = mult * 10;
        println!("{}", no1);
    }
    return no1;
}

pub fn reverse_sum(sum:i64) -> i64 {
//    let vec:Vec<i8> = Vec::new();
    let sum_string = sum.to_string();
    let sum_string_rev = sum_string.chars().rev().collect::<String>();
    let sum_rev = sum_string_rev.parse::<i64>().unwrap();
    //   println!("{}", sum_rev);
    return sum_rev;
}

// https://users.rust-lang.org/t/how-to-convert-a-number-to-numeric-vec/10404
pub fn vectorize_sum(sum:i64) -> Vec<i64> {
    let mut vec_sum = Vec::new();
    let mut sum = sum;
    while sum > 9 {
        vec_sum.push(sum % 10);
        sum = sum / 10;
    }
    vec_sum.push(sum);
    vec_sum.reverse();

    println!("vec_sum is {:?}", vec_sum);
    return vec_sum;
}


fn main(){
    let reconstructed_first = Vec::new();
    let reconstructed_second = Vec::new();

    let l1 = Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None }))}))}));
    let l2 = Some(Box::new(ListNode{ val: 5, next: Some(Box::new(ListNode{ val: 6, next: Some(Box::new(ListNode { val: 4, next: None }))}))}));
    //println!("{:?}", l1);
    let add1 = ListNode::split_list(l1, reconstructed_first);
    let add2 = ListNode::split_list(l2, reconstructed_second);
    println!("{:?}", add1);
    println!("{:?}", add2);

    let number1:i64 = combine_vector(add1);
    let number2:i64 = combine_vector(add2);

    println!("{:?}", number1 + number2);
    let sum:i64 = number1 + number2;

    let z = vectorize_sum(sum);

    let tail = Some(Box::new(ListNode{val:z[0], next:None}));
    let next = Some(Box::new(ListNode{val:z[1],next:tail}));
    let head = Some(Box::new(ListNode{val:z[2],next:next}));

    println!("head is {:?}",head);
    return;
}