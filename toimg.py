import numpy as np
import matplotlib.pyplot as plt

# img = np.zeros((VERTICAL_PRECISION, HORIZONTAL_PRECISION), dtype=np.uint8)
with open ("dump") as f:
    out = []
    for line in f.read().split("\n")[:-1]:
        l = []
        for j in line.split(",")[:-1]:
            l.append(int(j))
        out.append(l)

img = np.array(out, dtype=np.uint8)

plt.imshow(img, cmap="gray")
plt.show()
