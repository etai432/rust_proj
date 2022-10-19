//not working :/
// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     if nums.len() == 1 || (nums.len() == 2 && nums[0] == nums[1]) {
//         return 1;
//     }
//     if nums.len() == 2 {
//         return 2;
//     }
//     let mut next: i32 = nums[1];
//     let mut index: i32 = 0;
//     let mut x = 0;
//     let len = nums.len();
//     while &x < &(nums.len() - 2) {
//         if next == nums[x] {
//             nums.push(nums[x]);
//             nums.remove(x);
//             index += 1;
//             next = nums[x+1];
//             if nums[x] == nums[x+1] && (nums.len() as i32 - index != x as i32) {
//                 index -= 1;
//             }
//         } else {
//             x += 1;
//             next = nums[x+1];
//         }
//     }
//     return nums.len() as i32 - index;
// }