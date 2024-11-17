# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "numpy",
#   "pandas",
#   "matplotlib",
#   "scipy",
# ]
# ///


"""Treasury rates"""

"""
# https://quantdare.com/interest-rates-modeling/#:~:text=What%20is%20the%20Nelson%2DSiegel,of%20the%20yield%20curve's%20behavior.
# https://medium.com/@pape14/understanding-the-nelson-siegel-svensson-nss-model-for-bond-yield-curve-analysis-2a23202cbf6b
# https://github.com/MoQuant/NSModel/tree/main

Nelson-Siegal formula used to model yield curve, which captures the relationship between 
bond yields and maturities. It is good for it's ability to fit different shapes of the yield curve
(upward, downward, humped)

Yield curve factor models are based on the idea that the yield curve can be decomposed into several components, 
each of which describes a different aspect of the yield curve’s behavior.

This model is often used for interest rate forecasting and curve fitting in bond pricing.

The NS model essentially uses 3 smoothing parameters, to describe the shape of the yield curve. 
The first parameter, β0 describes the long-term level of interest rates. 
The second parameter, β1 describes the slope.
The third parameter,β2 describes the curvature of the yield curve.

There are 2 extensions to this model which help to overcome its weaknesses:
1. Dynamic Nelson-Siegel model(DNS)
2. Svensson extension (NSS).

There are other models like cubic splines, b-splines, exponential-splines

The outputs from the model can be the zero coupon curve (zero coupon rates against time), 
par curve (yields and coupon rates of par bonds against time), 
or forward curve (forward short-term interest rates). 
These curves are mathematical transformations of each other. From a single model, you automatically get all of them.

Conceptually, the term-structure of interest rates (spot rate curve) is calculated with zero coupon treasuries. 
Nelson-Siegel will accomplish this.

The yield curve is calculated with treasury Bonds, which pay coupons. 
While this is a good benchmark for examining the current state of interest rates, you will not gain much by comparing corporate bonds to this curve (since the coupons vary). Additionally, there are discrepancies with on the run issues, and how the day count is calculated with respect to corporates.

It is for that reason that you want to model the zero coupon term-structure (spot rate curve). 
This will strip out many of the nuances of treasury bonds and give you a clearer picture on the spread between your corporate bond and zero-coupon treasuries.
"""

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.optimize import minimize


def NelsonSiegel(b0: int, b1: int, b2: int, lb: int, t: int) -> None:
    """We break down the formula."""
    A = (1 - np.exp(-t / lb)) / (t / lb)
    B = A - np.exp(-t / lb)
    return b0 + b1 * A + b2 * B


def Optimizer(y: int, t: int) -> None:
    """"""
    def Objective(x, actual, maturity):
        ns = NelsonSiegel(x[0], x[1], x[2], x[3], maturity)
        return np.sum((actual - ns) ** 2)

    x = [0.1, 0.1, 0.1, 0.1]
    res = minimize(Objective, x, args=(y, t))
    return res.x


# we want to load the dataset where the oldest is the last and newest is the first
dataset: pd.DataFrame = pd.read_csv('trates.csv')[::-1]

# extract the dates
dates = dataset['Date'].values
del dataset['Date']

# turn to list which will be used in the plots.
mats: list = dataset.columns.tolist()

# normalize the values
dataset = dataset.values / 100.0

# create the 3d plot alongside the model plot
fig = plt.figure(figsize=(10, 5))
ax = fig.add_subplot(121, projection='3d')
ay = fig.add_subplot(122)

# get the latest treasury yields as we will forecast
Y = dataset[-1]

# we divide by 12 to get tenor in years.
T = np.array([1, 2, 3, 4, 6, 12, 24, 36, 60, 84, 120, 240, 360]) / 12.0

b0, b1, b2, lb = Optimizer(Y, T)

YForecast = NelsonSiegel(b0, b1, b2, lb, T)

# plot the NS model where red is the actual treasury yield curve and the blue is forecasted
ay.plot(range(len(mats)), Y, color='red')
ay.plot(range(len(mats)), YForecast, color='blue')
ay.set_xticks(range(len(mats)))
ay.set_xticklabels(mats)

# create the 3d surface plot
m, n = dataset.shape
print(m, n)
# we use meshgrid to create tuple of co-ordinates
x, y = np.meshgrid(range(m), range(n))

ax.plot_surface(x, y, dataset.T, cmap='jet')
ax.set_yticks(list(range(len(mats))))
ax.set_yticklabels(mats, rotation=35)

plt.show()
