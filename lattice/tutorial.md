# Definitions

**Euclidean Norm**

It is used to measure a vector length in cryptography circumstances, assuming there is a vector $\bold{v} = (v_0, v_1, ..., v_{n - 1}) \in \mathbb{R}^n$, then its Euclidean Norm is:
$$
\Vert \bold{v} \Vert_2 = \sqrt{v_0^2 + v_1^2 + ... + v_{n - 1}^2}
$$

Key Properties:
- Non-negativity, if $\Vert \bold{v} \Vert_2 = 0$, then we must have $\bold{v} = 0$.
- Scalar-multiplicity, $\Vert k \bold{v} \Vert_2 = k \cdot \Vert v \Vert_2$ holds for any scalars $k$.
- Triangle Inequality, $\Vert \bold{u} + \bold{v} \Vert_2 \ge \Vert \bold{v} \Vert_2 + \Vert \bold{u} \Vert_2$.

