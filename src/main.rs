// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
#[derive(Debug)]
pub struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

fn split_list(l1:Option<Box<ListNode>>, mut reconstructed_number: Vec<i32>) -> Vec<i32> {

    let l1unwrap = l1.as_ref().unwrap();
    //   println!("{:?}", l1unwrap.val);
    reconstructed_number.push(*&l1unwrap.val);
//        println!("{:?}", l1unwrap.next);
    if l1unwrap.next == None {return reconstructed_number;}
    let nextnode= l1unwrap.next.to_owned();
    split_list(nextnode, reconstructed_number)
}

pub fn combine_vector(vec: Vec<i32>) -> u128 {
    let mut no1:u128 = 0;
    let mut mult:u128 = 1;
    for num in vec {
        no1 = no1 + ( num as u128 * mult);
        mult = mult * 10;
        println!("{}", no1);
    }
    return no1 as u128;
}

pub fn reverse_sum(sum:i32) -> i32 {
//    let vec:Vec<i8> = Vec::new();
    let sum_string = sum.to_string();
    let sum_string_rev = sum_string.chars().rev().collect::<String>();
    let sum_rev = sum_string_rev.parse::<i32>().unwrap();
    //   println!("{}", sum_rev);
    return sum_rev;
}

// https://users.rust-lang.org/t/how-to-convert-a-number-to-numeric-vec/10404
pub fn vectorize_sum(sum:u128) -> Vec<i32> {
    let mut vec_sum:Vec<i32> = Vec::new();
    let mut sum = sum;
    while sum > 9 {
        let sumclock10 = sum%10;
        vec_sum.push(sumclock10 as i32);
        sum = sum / 10;
    }
    vec_sum.push(sum as i32);
    vec_sum.reverse();

    //   println!("vec_sum is {:?}", vec_sum);
    return vec_sum;
}


//impl Solution {
//    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
fn main(){
    let l1 = Some(Box::new(ListNode{ val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None }))}))}));
    let l2 = Some(Box::new(ListNode{ val: 5, next: Some(Box::new(ListNode{ val: 6, next: Some(Box::new(ListNode { val: 4, next: None }))}))}));
        let reconstructed_first = Vec::new();
        let reconstructed_second = Vec::new();

        let add1 = split_list(l1, reconstructed_first);
        let add2 = split_list(l2, reconstructed_second);
        println!("add2 = {:?}", add2);

        let number1:u128 = combine_vector(add1);
        println!("number 1 = {:?}", number1);
        let number2:u128 = combine_vector(add2);
        println!("number 2 = {:?}", number2);

        let sum:u128 = number1 + number2;

        let z = vectorize_sum(sum);

        let mut list = List::new();

        for v in z {
            list.push(v);
        }

        println!("{:?}",list.head);
 //       list.head
    }
//}
