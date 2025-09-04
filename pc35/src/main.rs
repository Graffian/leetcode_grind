pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut mid = (start+end)/2;

    while start < end && start !=mid{
        if nums[mid] == target{
            return mid as i32
        }else if nums[mid] < target{
            start = mid + 1;
        }else if nums[mid] > target{
            end = mid - 1;   
        }
        mid = (start+end)/2;
}
    if nums[start] >= target{
        return start as i32
    }
    if nums[end] < target && nums[end] != target{
        return (end+1) as i32
    }else{
        return end as i32
    }
}


fn main() {
    let nums = vec![2,5];
    let target = 5;
    let index = search_insert(nums, target);
    println!("THE REQD INDEX IS : {}" , index);
}
