import itertools as it

InputList = []
with open("data/q24.data", "r") as data:
    for t in data:
        P, V = t.strip().split(" @ ")
        PX, PY, PZ = list(map(int, P.split(", ")))
        VX, VY, VZ = list(map(int, V.split(", ")))
        NewTuple = (PX, PY, PZ, VX, VY, VZ)
        InputList.append(NewTuple)

#PX1 + VX1*t = PX2 + VX2*t
#PX1 - PX2 = VX2*t - VX1*t
#(PX1-PX2)/(VX2-VX1) = t

#y = mx + b
#m = avy/avx
#b = apy - m*apx
#m1x + b1 = m2x + b2
#m1x - m2x = b2 - b1
#x = (b2-b1)/(m1-m2)
#200000000000000
#400000000000000
        
NumHails = len(InputList)
Part1Answer = 0
NumCombos = 0
Min = 200000000000000
Max = 400000000000000
InputList.sort()
for A, B in it.combinations(InputList, 2):
    NumCombos += 1
    APX, APY, APZ, AVX, AVY, AVZ = A
    BPX, BPY, BPZ, BVX, BVY, BVZ = B
    MA = (AVY/AVX)
    MB = (BVY/BVX)
    CA = APY - (MA*APX)
    CB = BPY - (MB*BPX)
    if MA == MB:
        continue
    XPos = (CB-CA)/(MA-MB)
    YPos = MA*XPos + CA
    if (XPos < APX and AVX > 0) or (XPos > APX and AVX < 0) or (XPos < BPX and BVX > 0) or (XPos > BPX and BVX < 0):
        continue
    if Min <= XPos <= Max and Min <= YPos <= Max:
        Part1Answer += 1




PotentialXSet = None
PotentialYSet = None
PotentialZSet = None
for A, B in it.combinations(InputList, 2):
    APX, APY, APZ, AVX, AVY, AVZ = A
    BPX, BPY, BPZ, BVX, BVY, BVZ = B

    if AVX == BVX and abs(AVX) > 100:
        NewXSet = set()
        Difference = BPX - APX
        for v in range(-1000, 1000):
            if v == AVX:
                continue
            if Difference % (v-AVX) == 0:
                NewXSet.add(v)
        if PotentialXSet != None:
            PotentialXSet = PotentialXSet & NewXSet
        else:
            PotentialXSet = NewXSet.copy()
    if AVY == BVY and abs(AVY) > 100:
        NewYSet = set()
        Difference = BPY - APY
        for v in range(-1000, 1000):
            if v == AVY:
                NewYSet.add(v) #New line
                continue
            if Difference % (v-AVY) == 0:
                NewYSet.add(v)
        if PotentialYSet != None:
            PotentialYSet = PotentialYSet & NewYSet
        else:
            PotentialYSet = NewYSet.copy()
    if AVZ == BVZ and abs(AVZ) > 100:
        NewZSet = set()
        Difference = BPZ - APZ
        for v in range(-1000, 1000):
            if v == AVZ:
                continue
            if Difference % (v-AVZ) == 0:
                NewZSet.add(v)
        if PotentialZSet != None:
            PotentialZSet = PotentialZSet & NewZSet
        else:
            PotentialZSet = NewZSet.copy()

print(PotentialXSet, PotentialYSet, PotentialZSet)
RVX, RVY, RVZ = PotentialXSet.pop(), PotentialYSet.pop(), PotentialZSet.pop()

APX, APY, APZ, AVX, AVY, AVZ = InputList[0]
BPX, BPY, BPZ, BVX, BVY, BVZ = InputList[1]
MA = (AVY-RVY)/(AVX-RVX)
MB = (BVY-RVY)/(BVX-RVX)
CA = APY - (MA*APX)
CB = BPY - (MB*BPX)
XPos = int((CB-CA)/(MA-MB))
YPos = int(MA*XPos + CA)
Time = (XPos - APX)//(AVX-RVX)
ZPos = APZ + (AVZ - RVZ)*Time

print(XPos, YPos, ZPos)
Part2Answer = XPos + YPos + ZPos


print(f"{Part1Answer = }")
print(f"{Part2Answer = }")