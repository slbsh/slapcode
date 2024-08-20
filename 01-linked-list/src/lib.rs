#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(
    clippy::option_map_unit_fn,
    clippy::wrong_self_convention,
    clippy::uninit_assumed_init,
    clippy::new_without_default,
    non_camel_case_types
)]

#[repr(transparent)]
pub struct List<λ>(*mut Node<λ>); // power of lambda!

pub struct Node<λ> {
    next: *mut Node<λ>,
    data: λ,
}

impl<λ> From<Vec<λ>> for List<λ> {
    fn from(v: Vec<λ>) -> Self {
        let mut head = None::<Node<λ>>;

        for item in v.into_iter().rev() {
            let prev = head
                .take()
                .map_or_else(std::ptr::null, |prev| Box::into_raw(Box::new(prev)));

            let node = Node {
                next: prev as *mut Node<λ>,
                data: item,
            };

            head = Some(node)
        }

        List(head.map_or_else(std::ptr::null, |head| Box::into_raw(Box::new(head))) as *mut Node<λ>)
    }
}

impl<λ: Ord> List<λ> {
    pub fn sort(&mut self) {
        loop {
            let mut needed_sort = false;

            let node = unsafe { self.0.as_mut() };

            let Some(mut node) = node else {
                return;
            };

            while let Some(next) = unsafe { node.next.as_mut() } {
                if node.data > next.data {
                    let next_next = std::mem::replace(&mut next.next, std::ptr::null_mut());
                    unsafe {
                        std::ptr::swap(node, next);
                    };
                    next.next = next_next;
                    node.next = next;
                    needed_sort = true;
                } else {
                    node = next
                }
            }

            if !needed_sort {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test;
