use std::{cmp::Reverse, iter::once_with};
use priority_queue::PriorityQueue;

#[derive(Debug,Clone)]
pub struct HuffmanTree {
    pub root: HuffmanNode
}

impl HuffmanTree {
    pub fn new() -> HuffmanTree {
        HuffmanTree {
            root: HuffmanNode::new_empty(),
        }
    }
    pub fn create_dict(self) -> [Vec<bool>;256] {
        let mut arr: [Vec<bool>;256] = (0..256).into_iter().map(|_| {
            Vec::new()
        }).collect::<Vec<_>>().try_into().unwrap();

        Self::find_leaf(&self.root, vec![], &mut arr);
        arr
    }
    fn find_leaf(node: &HuffmanNode, mut current_path: Vec<bool>, arr: &mut [Vec<bool>;256]) {
        match node.value {
            Some(x) => arr[x as usize] = current_path,
            None => {
                let mut new_path = current_path.clone();
                new_path.push(false);
                Self::find_leaf(&node.next[0],new_path,arr);
                current_path.push(true);
                Self::find_leaf(&node.next[1],current_path,arr)
            }
        }
    }
    pub fn reverse_dict(arr: &[Vec<bool>;256]) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![0; 32768];
        arr.iter().enumerate().for_each(|(value, path)| {
            let path = path.iter().chain(once_with(|| &true)).rev().fold(0_u16, |num_path,bool_path| {
                if *bool_path {
                    (num_path | 1) << 1
                } else {
                    num_path << 1
                }
            }) >> 1;
            vec[path as usize] = value as u8;
        });
        vec
    }
    pub fn reverse_dict_get(reverse_dict: &Vec<u8>, path: &Vec<bool>) -> u8 {
        let path = path.iter().chain(once_with(|| &true)).rev().fold(0_u16, |num_path,bool_path| {
            if *bool_path {
                (num_path | 1) << 1
            } else {
                num_path << 1
            }
        }) >> 1;
        reverse_dict[path as usize] as u8
    } 

    pub fn decode(node: &HuffmanNode, path: &[bool]) -> Option<u8> {
        match node.value {
            Some(_) => node.value,
            None if path.len() > 0 => {
                match path[0] {
                    true => Self::decode(&node.next[1],&path[1..]),
                    false => Self::decode(&node.next[0],&path[1..])
                }
            }
            _ => None
        }
    }

    pub fn create_tree_arr(data:&Vec<u8>) -> HuffmanTree {
        let mut dict: [u64; 256] = [0; 256];

        data.iter().for_each(|&x| {
            dict[x as usize] += 1;
        });

        Self::create(dict)
    }
    fn create(dict: [u64; 256]) -> HuffmanTree {
        let mut queue: PriorityQueue<HuffmanNode, Reverse<u64>> = PriorityQueue::new();

        dict.into_iter().enumerate().for_each(|(value,priority)| {
            queue.push(HuffmanNode::new(Some(value as u8), vec![]),Reverse(priority));
        });

        while queue.len() > 1 {
            let node1 = queue.pop().unwrap();
            let node2 = queue.pop().unwrap();

            let priority = node1.1.0 + node2.1.0;

            let root = HuffmanNode::new(None, vec![node1.0,node2.0]);
            queue.push(root, Reverse(priority));

        };
        HuffmanTree { root: queue.pop().unwrap().0 }
    }
}
#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct HuffmanNode {
    value: Option<u8>,
    next: Vec<HuffmanNode>,
}

impl HuffmanNode {
    fn new(value: Option<u8>, next:Vec<HuffmanNode>) -> HuffmanNode {
        HuffmanNode {
            value: value,
            next
        }
    }
    fn new_empty() -> HuffmanNode {
        HuffmanNode {
            value: None,
            next: vec![]
        }
    }
    
}