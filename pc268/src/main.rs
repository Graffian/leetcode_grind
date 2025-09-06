
pub fn missing_number(mut nums: Vec<i32>) -> i32 {
    let mut i=0;
    let mut j = 1;
    let mut swapper: i32 = 0;
    while i<=nums.len()-1{
        while j<=nums.len()-1{
            if nums[i] < nums[j]{
                swapper = nums[j];
                nums[j] = nums[i];
                nums[i] = swapper;
            }
            j=j+1;
        }
        i=i+1;
        j= 0;
    };
    let mut k:usize = 0;
    while k<=nums.len()-1{
        if k as i32 != nums[k]{
            break
        }
        k = k+1; 
    };
    k as i32
}


fn main() {
    let nums = vec![9,6,4,2,3,5,7,0,1];
    let missing_num = missing_number(nums);
    println!("the missing number is {}" , missing_num);
}
