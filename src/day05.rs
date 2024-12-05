use std::collections::{HashMap, HashSet};

pub fn task1(orderings: &Vec<(i32, i32)>, prints: &Vec<Vec<i32>>) -> i32 {
    let mut orderings_map = HashMap::<i32, Vec<i32>>::new();
    orderings
        .iter()
        .for_each(|(prev, next)| orderings_map.entry(*prev).or_default().push(*next));
    prints
        .iter()
        .map(|pages| {
            if validate_pages(pages, &orderings_map) {
                pages[pages.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

pub fn task2(orderings: &Vec<(i32, i32)>, prints: &Vec<Vec<i32>>) -> i32 {
    let mut orderings_map = HashMap::<i32, Vec<i32>>::new();
    let mut dependency_map = HashMap::<i32, Vec<i32>>::new();
    orderings.iter().for_each(|(prev, next)| {
        orderings_map.entry(*prev).or_default().push(*next);
        dependency_map.entry(*next).or_default().push(*prev);
    });
    prints
        .iter()
        .map(|pages| {
            if !validate_pages(pages, &orderings_map) {
                let reordered_pages = reorder_pages(pages, &orderings_map);
                reordered_pages[reordered_pages.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn reorder_pages(pages: &Vec<i32>, dependency_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let page_set = HashSet::<i32>::from_iter(pages.iter().cloned());
    let mut page_dependency_map = HashMap::<i32, HashSet<i32>>::new();
    page_set.iter().for_each(|page| {
        page_dependency_map.insert(
            *page,
            HashSet::from_iter(dependency_map.get(page).unwrap().iter().cloned())
                .intersection(&page_set)
                .cloned()
                .collect(),
        );
    });

    let mut reordered_pages = Vec::<i32>::new();
    while !page_dependency_map.is_empty() {
        // Find the next page without any dependencies
        let next_page = *(page_dependency_map
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .next()
            .unwrap()
            .0);
        reordered_pages.push(next_page);
        page_dependency_map
            .iter_mut()
            .for_each(|(_, dependencies)| {
                dependencies.remove(&next_page);
            });
        page_dependency_map.remove(&next_page);
    }
    reordered_pages
}

fn validate_pages(pages: &Vec<i32>, orderings: &HashMap<i32, Vec<i32>>) -> bool {
    let mut seen = HashSet::<i32>::new();
    for page in pages.iter() {
        if let Some(conditions) = orderings.get(page) {
            if conditions.iter().any(|next| seen.contains(next)) {
                return false;
            }
        }
        seen.insert(*page);
    }
    true
}
