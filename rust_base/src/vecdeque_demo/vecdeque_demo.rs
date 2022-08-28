use std::collections::{TryReserveError, VecDeque};

//创建一个空的队列

#[test]
fn test_create_empty_deque() {
    let deque: VecDeque<u32> = VecDeque::new();
}

//创建一个至少包含空间的空元素。capacity

#[test]
fn test_create_empty_deque_with_capacity() {
    let deque: VecDeque<u32> = VecDeque::with_capacity(10);
}

//get 提供对给定索引处的元素的引用

#[test]
fn test_get() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_back(4);

    assert_eq!(deque.get(0), Some(&1));
}

//get_mut 提供对给定索引处的元素的可变引用
#[test]
fn test_get_mut() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_back(4);

    if let Some(elem) = deque.get_mut(1) {
        *elem = 7;
    }
    assert_eq!(deque[1], 7);
}

//在指定索引处交换ij元素。i并且可能相等
#[test]
fn test_swap() {
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf, [3, 4, 5]);
    buf.swap(0, 2);
    assert_eq!(buf, [5, 4, 3]);
}

//返回 deque 在不重新分配的情况下可以容纳的元素数
#[test]
fn test_capacity() {
    let buf: VecDeque<i32> = VecDeque::with_capacity(10);
    assert!(buf.capacity() >= 10);
}

//保留最小容量，以便至少将更多元素插入到给定的截面中
#[test]
fn test_reserve_exact() {
    let mut buf: VecDeque<i32> = [1].into();
    buf.reserve_exact(10);
    assert!(buf.capacity() >= 11);
}

//为至少更多元素保留在给定的截面中插入的容量
#[test]
fn test_reserve() {
    let mut buf: VecDeque<i32> = [1].into();
    buf.reserve(10);
    assert!(buf.capacity() >= 11);
}

//尝试为至少更多元素保留在给定的 deque 中所需的最小容量
fn test_try_reserve_exact(data: &[u32]) -> Result<VecDeque<u32>, TryReserveError> {
    let mut output = VecDeque::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve_exact(data.len())?;

    // Now we know this can't OOM(Out-Of-Memory) in the middle of our complex work
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // very complicated
    }));

    Ok(output)
}

//尝试为至少要在给定的 deque 中插入更多元素保留容量
fn test_try_reserve(data: &[u32]) -> Result<VecDeque<u32>, TryReserveError> {
    let mut output = VecDeque::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // very complicated
    }));

    Ok(output)
}

//缩短 deque，保留第一个元素并删除其余元素。len
#[test]
fn test_truncate() {
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(10);
    buf.push_back(15);
    assert_eq!(buf, [5, 10, 15]);
    buf.truncate(1);
    assert_eq!(buf, [5]);
}
