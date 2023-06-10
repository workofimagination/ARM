import matplotlib.pyplot as plt;

nums = open("./temp").read();
nums = nums.split(" ");

#nums = [x for x in range(1000)];
#smoothed = list();
#mi = nums[0];
#ma = nums[-1];
#
#for num in nums:
#    k = max(0, min(1, (num-mi)/(ma-mi)));
#    t = (6*k) - (6*k*k);
#
#    smoothed.append(t);

print(max(nums));

plt.plot(nums);
plt.show();



