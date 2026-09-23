class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        pairs = {
            ')': '(',
            '}': '{',
            ']': '['
        }

        for b in s:
            if b in "({[":
                stack.append(b)
            else:
                if not stack or stack.pop() != pairs[b]:
                    return False

        return len(stack) == 0