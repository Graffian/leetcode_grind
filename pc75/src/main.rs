pub fn sort_colors(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut swapper = 0;
    let mut i=0;
    let mut j = 1;
    while i<=nums.len() - 1{
        while j<=nums.len()-1{
            if nums[i] < nums[j]{
                swapper = nums[i];
                nums[i] = nums[j];
                nums[j] = swapper;
            }
            j = j+1;
        }
        i = i + 1;
        j=0;
    }
    return nums.to_vec()
}

fn main() {
    let mut nums = vec![2,0,2,1,1,0]; // output sd be [0,0,1,1,2,2]
    let sorted_nums = sort_colors(&mut nums);
    println!("THE SORTED colors are {:?}" , sorted_nums);

}
