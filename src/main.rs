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


fn add_lists(l1:Option<Box<ListNode>>, l2:Option<Box<ListNode>>, mut vec:Vec<u8>, carry: i32) -> Vec<u8> {
    let carry_copy = carry;
    let mut carry= 0;
    let mut l1unwrap:&Box<ListNode> = &Box::new(ListNode { val: 0, next: None });
    let mut l2unwrap:&Box<ListNode> = &Box::new(ListNode { val: 0, next: None });
    if l1 != None {
        l1unwrap = l1.as_ref().unwrap();
    }
    if l2 != None {
        l2unwrap = l2.as_ref().unwrap();
    }
    println!("Linked list 1 members are {:?}", l1unwrap.val);
    println!("Linked list 2 members are {:?}", l2unwrap.val);
    let mut individual_sum = l1unwrap.val + l2unwrap.val + carry_copy;
//    reconstructed_number.push(*&l1unwrap.val);
//        println!("individual sum is {:?}", individual_sum);
    if individual_sum > 9 {
        carry = 1;
        individual_sum = individual_sum % 10;
    }
    println!("individual sum is {:?}", individual_sum);
    vec.push(individual_sum as u8);
    //if l1unwrap.next == None && l2unwrap.next == None {return l3;}
    //if l2unwrap.next == None {return l3;}
    let nextnode1= l1unwrap.next.to_owned();
    let nextnode2= l2unwrap.next.to_owned();
    if nextnode1 == None && nextnode2 == None && carry == 0 {return vec;}
    add_lists(nextnode1, nextnode2, vec, carry)
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

    let vec: Vec<u8> = Vec::new();

    let mut ans = add_lists(l1, l2, vec, 0);

    println!("{:?}", ans);
    ans.reverse();
    println!("{:?}", ans);

    let mut l3 = List::new();
    for a in ans {
        l3.push(a as i32);
    }

    println!("{:?}",l3.head);

}
