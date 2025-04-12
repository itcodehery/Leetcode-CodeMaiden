import 'dart:math';

class Solution {
  List<int> twoSum(List<int> nums, int target) {
    if (2 <= nums.length &&
        nums.length <= pow(10, 4) &&
        pow(-10, 9) <= target &&
        target <= pow(10, 9)) {
      for (int i = 0; i < nums.length; i++) {
        for (int j = i + 1; j < nums.length; j++) {
          if (nums[i] + nums[j] == target) {
            return [i, j];
          }
        }
      }
    }
    return [];
  }
}

void main() {
  print(Solution().twoSum([2, 7, 11, 15], 9));
}
