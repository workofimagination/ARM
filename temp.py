import matplotlib.pyplot as plt;

linear = [x for x in range(1000)];

smoothed = list();

max_linear = max(linear);
min_linear = min(linear);

for i in linear:
    k = max(0, min(1, (i-min_linear) / (max_linear - min_linear)));
    t = (6*k) - (6*k*k);
    
    smoothed.append(t);

plt.plot(smoothed);
plt.show();
