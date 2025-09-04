pub fn search(nums: &mut Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut mid = (start + end)/2;

    while start <= end{
        if nums[start] == nums[mid] && nums[start] != target && nums[mid] != target && nums[end] != target{
            return -1;
        }
        if nums[mid] == target{
            return mid.try_into().unwrap()
        }else if nums[mid] < target {
            start = mid + 1;
        }else if nums[mid] > target{
            end = mid - 1;
            
        }if nums[end] == target{
            return end.try_into().unwrap() 
        }

        mid = (start + end)/2;
    }

    if nums[start] == target{
        return start.try_into().unwrap()
    }

    return -1
}

fn main(){
    let mut nums = vec![1,3,5,6];
    let target = 7;
    let index = search(&mut nums, target);
    println!("REQUIRED INDEX : {}" , index);
}
