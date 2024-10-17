import math
import itertools

def judgePoint24(nums):
    if len(nums) == 1:
        return math.isclose(nums[0], 24)
    
    jugdes = []
    for a, b, *rest in itertools.permutations(nums):
        print(a, b)
        for x in {a+b, a-b, a*b, b and a/b}:
            jugde = judgePoint24([x] + rest)
            jugdes.append(jugde)
            print('========')


ret = judgePoint24([2,3,4,5])
