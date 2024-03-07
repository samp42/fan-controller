# generates a sine wave (TODO: that travels vertically)

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

dt = 0.05 # s
f = 1 # Hz

time_values = np.arange(0, 1 / f, dt)

df = pd.DataFrame({'dt': dt * np.ones_like(time_values)})

y = []

for t in time_values:
    y.append(int(-50 * np.cos(2 * np.pi * t) + 50))

for i in range(1, 82):
    fan_label = f'fan{i}'
    df[fan_label] = y

df.to_csv('pattern1.csv', index=False)

plt.plot(time_values, y)
plt.show()
