import numpy as np

M = np.fromregex('../../input/day13.txt', r'\d+', [('', int)]
    *6).view(int).reshape(-1,3,2).swapaxes(1,2)

for p in 0, 1e13:
    S = M[..., :2]
    P = M[..., 2:] + p
    R = np.linalg.solve(S, P).round().astype(int)
    print(*R.squeeze() @ [3,1] @ (S @ R == P).all(1))