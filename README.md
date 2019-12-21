# Basic PBT Demo
Implementation of the basic example of PBT in Rust from https://arxiv.org/abs/1711.09846

## Rough Overview
The basic example from the article tries to optimize a function ![equation](https://latex.codecogs.com/gif.latex?1.2&space;-&space;(h_0&space;\theta_0^2&plus;h_1&space;\theta_1^2)) where ![equation2](https://latex.codecogs.com/gif.latex?\vec{h}) represents the hyperparameters. 
The objective function that we are trying to optimize against is ![equation3](https://latex.codecogs.com/gif.latex?1.2&space;-&space;(\theta_0^2&space;&plus;&space;\theta_1^2))
This code uses PBT as described in the paper with the `exploit` step being to choose the optimal `h` and `theta` from among the threads, and the `explore` step being modifying the parameters by adding a value drawn from normal distribution with mean `0` and standard deviation `0.1`.

## Run 
Run with by running `cargo run` in the main directory.
