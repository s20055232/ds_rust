use rand::Rng;

struct MyArray;

#[allow(dead_code, unused_variables)]
fn main() {
    // initialize a array
    // generate 5 numbers of zero
    let arr: Vec<u32> = vec![0; 5];
    let nums: Vec<i32> = vec![1, 3, 2, 5, 4];
}

#[allow(dead_code, unused_variables)]
impl MyArray {
    /// In array, randomly access element is very quick, it only take O(1).
    fn random_access(nums: &[i32]) -> i32 {
        let random_index = rand::thread_rng().gen_range(0..nums.len());
        // 官方解法
        // let random_num = nums[random_index];
        // random_num

        // 更好的作法
        nums[random_index]
    }
    /// 刪除其實就是將該索引後面的數字往前移動，將該位置的數值覆蓋
    fn remove(nums: &mut Vec<i32>, index: usize) {
        for i in index..nums.len() - 1 {
            nums[i] = nums[i + 1];
        }
    }
    /// 可以透過索引遍歷，也可以直接遍歷
    fn traverse(nums: &[i32]) {
        let mut _count = 0;
        for _ in 0..nums.len() {
            _count += 1;
        }
        for _ in nums {
            _count += 1;
        }
    }
    /// 在array的搜尋中，需要遍歷每個元素來找到相對應的數值，由於有可能找不到，因此使用Option，回傳數值或是None
    fn find(nums: &[i32], target: i32) -> Option<usize> {
        // 官方解法
        // for i in 0..nums.len() {
        //     if nums[i] == target {
        //         return Some(i);
        //     }
        // }
        // None

        // 更好的作法
        (0..nums.len()).find(|&x| nums[x] == target)
    }
    /// array擴充，當array數量達上限時，需要重新建立一個有更大的內存空間的array，並將舊的array放進去
    fn extend(nums: Vec<i32>, enlarge: usize) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums.len() + enlarge];
        // 官方解法
        // for i in 0..nums.len() {
        //     res[i] = nums[i];
        // }
        // res

        // 更好的作法
        res[..nums.len()].copy_from_slice(&nums[..]);
        res
    }
}
