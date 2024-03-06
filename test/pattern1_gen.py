# generates a sine wave that travels vertically

import numpy as np
import pandas as pd

time_values = np.arange(0, 2 * np.pi, 0.001)

df = pd.DataFrame({'dt': 0.001 * np.ones_like(time_values)})

# for t in time_values:
#     sine_wave = 100 * np.sin(t)
#     for i in range(1, 82):
#         fan_label = f'fan{i}'
#         df[fan_label] = sine_wave

# df.to_csv('pattern1.csv', index=False)

for t in time_values:
    print(np.sin(t / (2*np.pi)))