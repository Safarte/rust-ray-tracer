use aabb::AABB;
use rand::{thread_rng, Rng};

use crate::{bvh::aabb::aabb_compare, ray::Ray};

use self::aabb::surrounding_box_vec;

pub mod aabb;

pub trait Bounded {
    fn aabb(&self) -> AABB;
}

pub enum BVHNode {
    Leaf {
        parent_index: usize,
        primitive_index: usize,
    },
    Node {
        parent_index: usize,
        child_l_index: usize,
        child_l_aabb: AABB,
        child_r_index: usize,
        child_r_aabb: AABB,
    },
}

impl BVHNode {
    fn build_rec<P: Bounded>(
        primitives: &[P],
        indices: &[usize],
        nodes: &mut Vec<BVHNode>,
        parent_index: usize,
    ) -> usize {
        let mut rng = thread_rng();
        let axis = rng.gen_range(0..2) as usize;

        if indices.len() == 1 {
            let primitive_index = indices[0];
            let node_index = nodes.len();
            nodes.push(BVHNode::Leaf {
                parent_index,
                primitive_index,
            });
            return node_index;
        } else {
            let mut aabbs = indices
                .iter()
                .map(|idx| primitives[*idx].aabb())
                .collect::<Vec<_>>();

            aabbs.sort_by(|a, b| aabb_compare(a, b, axis));

            let mid = indices.len() / 2;

            let mut left: Vec<usize> = vec![];
            let mut right: Vec<usize> = vec![];

            for idx in indices.iter() {
                if aabb_compare(&primitives[*idx].aabb(), &aabbs[mid], axis).is_lt() {
                    left.push(*idx);
                } else {
                    right.push(*idx);
                }
            }

            let node_index = nodes.len();

            // Dummy node
            nodes.push(BVHNode::Leaf {
                parent_index: 0,
                primitive_index: 0,
            });

            let child_l_aabb = surrounding_box_vec(
                &left
                    .iter()
                    .map(|idx| primitives[*idx].aabb())
                    .collect::<Vec<_>>(),
            );
            let child_r_aabb = surrounding_box_vec(
                &right
                    .iter()
                    .map(|idx| primitives[*idx].aabb())
                    .collect::<Vec<_>>(),
            );

            let child_l_index = BVHNode::build_rec(primitives, &left, nodes, node_index);
            let child_r_index = BVHNode::build_rec(primitives, &right, nodes, node_index);

            nodes[node_index] = BVHNode::Node {
                parent_index,
                child_l_index,
                child_l_aabb,
                child_r_index,
                child_r_aabb,
            };

            return node_index;
        }
    }

    fn traverse_rec(
        nodes: &[BVHNode],
        index: usize,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        indices: &mut Vec<usize>,
    ) {
        match nodes[index] {
            BVHNode::Node {
                ref child_l_aabb,
                child_l_index,
                ref child_r_aabb,
                child_r_index,
                ..
            } => {
                if child_l_aabb.hit(ray, t_min, t_max) {
                    BVHNode::traverse_rec(nodes, child_l_index, ray, t_min, t_max, indices);
                }
                if child_r_aabb.hit(ray, t_min, t_max) {
                    BVHNode::traverse_rec(nodes, child_r_index, ray, t_min, t_max, indices);
                }
            }
            BVHNode::Leaf {
                primitive_index, ..
            } => {
                indices.push(primitive_index);
            }
        }
    }
}

pub struct BVH {
    nodes: Vec<BVHNode>,
}

impl BVH {
    pub fn new<P: Bounded>(primitives: &[P]) -> Self {
        let indices = (0..primitives.len()).collect::<Vec<usize>>();
        let expected_node_count = primitives.len() * 2;
        let mut nodes = Vec::with_capacity(expected_node_count);
        BVHNode::build_rec(primitives, &indices, &mut nodes, 0);
        BVH { nodes }
    }

    pub fn traverse<'a, P: Bounded>(
        &'a self,
        ray: &Ray,
        primitives: &'a [P],
        t_min: f32,
        t_max: f32,
    ) -> Vec<&P> {
        let mut indices = Vec::new();
        BVHNode::traverse_rec(&self.nodes, 0, ray, t_min, t_max, &mut indices);
        indices
            .iter()
            .map(|index| &primitives[*index])
            .collect::<Vec<_>>()
    }
}
