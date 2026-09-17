class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        hmap = {}
        for x in nums:
            if x in hmap:
                return True
            else: 
                hmap[x] = 'a'
        return False
        