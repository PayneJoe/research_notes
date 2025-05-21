# Background

## Information

Information means uncertainity of a specific **event** (such as $P(X = x_0)$). How to measure it anyway? The information function

$$
f(x) = - \log x
$$

describes relation between **probability** ($x$-axis) and **information** ($y$=axis). It must satisfy two properties:

- it is a decreasing function within range $(0, 1]$, where $f(0) \approx +\infty$ , $f(1) = 0$.

- it satisfy addition law, $f(x \cdot y) = f(x) + f(y)$



For example given a specific **event probability** $P(X = x_0) = 1/4$, then the information is measured:

$$
f(P(X = x_0)) = - P(X = x_0) \cdot \log P(X = x_0) = - \frac{1}{4} \cdot \log \frac{1}{4} = \frac{1}{2}
$$

## 

## Entropy

Entropy means **expected** information, or **expected** uncertainity of an event $X$ with specified **probability distribution**. 



For example given a probability distribution of an event $X$, say:

| $X = x_i$    | $x_0$ | $x_1$ | $x_2$ | $x_3$ |
| ------------ | ----- | ----- | ----- | ----- |
| $P(X = x_i)$ | 1/4   | 0     | 1/2   | 1/4   |

Then the entropy (expected information) of event $X$ with some probability distribution $P(X = x_i)$ is:

$$
H(X) = -(2 \cdot \frac{1}{4} \cdot \log \frac{1}{4} + \frac{1}{2} \cdot \log \frac{1}{2}) = \frac{3}{2}
$$

## 

## Conditional Entropy

Conditional entropy means expected information of an event $Y$ knowing event $X$, where $Y$ is output event, $X$ is input event:

$$
\begin{aligned}
H(Y|X) &= \sum_{i} p_i \cdot H(Y|X = x_i) \\
&= \sum_{i} p_i \cdot \sum_{j} - p_{i, j} \cdot \log p_{i, j}
\end{aligned}
$$

For example, there is conditional probability matrix:

$$
p_{ij} = 
\begin{pmatrix} 
3/4 & 0 & 1/4 \\
0 & 1/2 & 1/2 \\
\end{pmatrix}
$$

assuming the input event $X$ probability distribution is $P(X = 0) = P(X = 1) = 1/2$, then the conditional entropy $H(Y|X) = - \frac{1}{2} \cdot (\frac{3}{4} \log \frac{3}{4} + \frac{1}{4} \log \frac{1}{4} + 2 \cdot \frac{1}{2} \log \frac{1}{2})$.



## Joint Entropy

Joint entropy means expected information gained from both input event $X$ (or $Y$) and output event $Y$ (or $X$):

$$
H(X, Y) = H(X) + H(Y|X) = H(Y) + H(X|Y)
$$



## Mutual Information

Mutual information measures information of output event $Y$ conveyed by input event $X$, or vice versa:

$$
I(X, Y) = H(Y) - H(Y|X) = H(X) - H(X|Y)
$$

Note that, **mutual information is not entropy**, it's the gap between the information of output event $Y$ and the information of output event $Y$ knowing input event $X$!



For example, according to above conditional probability matrix $p_{ij}$, the output event $Y$ probability is $P(Y = 0) = 3/8, P(Y = 1) = 1/4, P(Y = *) = 3/8$ . Then information of output event $H(Y) = - 2 \cdot \frac{3}{8} \log \frac{3}{8} - \frac{1}{4} \log \frac{1}{4}$. The mutual information can be calculated with:

$$
I(X, Y) = H(Y) - H(Y|X)
$$



But why this gap exists anyway? I think this is the meaning of information theory...






