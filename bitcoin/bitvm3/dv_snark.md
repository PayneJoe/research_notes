# Notations
- $n$ denotes the number of columns of a R1CS matrix, also the length of witness vector.

<br />

# Background

## Lagrange Interpolation

Assuming there is a domain $K = (k_0, k_1, k_2, ..., k_{n - 1})$, and a vector $\mathbf{y} = (y_0, y_1, y_2, ..., y_{n - 1})$. We can simulate a curve containing these $n$ points with **Lagrange Interpolation**.

First of all, according the determined domain $\mathbf{k}$, we can draw $n$ partial lagrange polynomials:

$$
L_i^K(X) = \prod_{j \ne i}^n \frac{X - k_j}{k_i - k_j}, i \in [n] \\
$$

whose degree is $n - 1$.

Then the lagrange polynomial is actually an inner product:

$$
p(X) = \langle L^{K}(X), \mathbf{y} \rangle
$$

<br />

## Pederson Commitment

Assuming there is a basis vector, say:

$$
\mathbf{b} = ([b_0]_1, [b_1]_1, [b_2]_1, ..., [b_{n - 1}]_1)
$$

We want to commit a polynomial whose coefficient vector is $\mathbf{x}$, then:

$$
c_{x} = \langle \mathbf{x}, \mathbf{b} \rangle
$$

In general, the elements of basis **Pederson Commitment** is discrete and random. But if the basis $\mathbf{b}$ is algebraic, say:

$$
\mathbf{b} = ([1]_1, [\tau]_1, [\tau^2]_1, ..., [\tau^{n - 1}]_1)
$$

then the **Pederson Commitment** for target polynomial would be:

$$
c_{p(X)} = [p(\tau)]_1 = [\langle L^K(\tau), \mathbf{y} \rangle]_1
$$

<br />

# DV-KZG

<br />

# Pari

In R1CS, there are two checks, **Lin-check** and **Row-check**.

## Lin-Check

$$
\mathbf{A} \mathbf{z} \overset{?}= \mathbf{z_A} \\
\mathbf{B} \mathbf{z} \overset{?}= \mathbf{z_B} \\
\mathbf{C} \mathbf{z} \overset{?}= \mathbf{z_C} \\
$$

With three random factors $\alpha, \beta, \gamma$, we can batch them together:

$$
(\alpha \cdot \mathbf{A} + \beta \cdot \mathbf{B} + \gamma \cdot \mathbf{C}) \mathbf{z} = \langle \alpha \cdot \mathbf{A} + \beta \cdot \mathbf{B} + \gamma \cdot \mathbf{C}, \mathbf{z} \rangle \overset{?}= \alpha \cdot \mathbf{z_A} + \beta \cdot \mathbf{z_B} + \gamma \cdot \mathbf{z_C}
$$

<br />

#### Setup

In setup phase, we need to prepare a new basis for committing witness vector $\mathbf{z}$:

$$
m_i(\tau) = \alpha \cdot \langle L^K(\tau), \mathbf{a_i} \rangle + \beta \cdot \langle L^K(\tau), \mathbf{b_i} \rangle + \gamma \cdot \langle L^K(\tau), \mathbf{c_i} \rangle, i \in [n] 
$$

where $\mathbf{a_i}, \mathbf{b_i}, \mathbf{c_i}$ are $i$-th columns of R1CS matrix $\mathbf{A}, \mathbf{B}, \mathbf{C}$ respectively, (Pederson) committing and batching them together constitutes a new basis $\mathbf{\Sigma_2}  = ([m_0(\tau)]_1, [m_1(\tau)]_1, [m_2(\tau)]_1, ..., [m_{n - 1}(\tau)]_1)$.

<br />

Along with lagrange basis $\mathbf{\Sigma_1} = ([L_0^K(\tau)]_1, [L_1^K(\tau)]_1, [L_2^K(\tau)]_1, ..., [L_{n - 1}^K(\tau)]_1)$ we have the commitment key specially for the purpose of commitments:

$$
ck = (\mathbf{\Sigma_1}, \mathbf{\Sigma_2})
$$

<br />

#### Run-time

**Commit Phase**

Firstly, prover needs to (Pederson) commit the right-hand target vectors $\mathbf{z_A}, \mathbf{z_B}, \mathbf{z_C}$ with basis $\Sigma_1$:

$$
c_{z_A} = \langle \mathbf{\Sigma_1}, \mathbf{z_A} \rangle \\
c_{z_B} = \langle \mathbf{\Sigma_1}, \mathbf{z_B} \rangle \\
c_{z_C} = \langle \mathbf{\Sigma_1}, \mathbf{z_C} \rangle \\
$$

<br />

Secondly, prover needs to (Pederson) commit the witness vector $z$ with basis $\Sigma_2$:

$$
c_z = \langle \mathbf{\Sigma_2}, \mathbf{z} \rangle
$$

<br />

One question about $c_z$, what if prover does not commit $\mathbf{z}$ with $\Sigma_2$? If he just fold $c_{z_A}, c_{z_B}, c_{z_C}$ with that three random $\alpha$s, since we all know that:

$$
c_z = \alpha_A \cdot c_{z_A} + \alpha_B \cdot c_{z_B} + \alpha_C \cdot c_{z_C}
$$

Actually this implies that, the prover does not need to without know witness $\mathbf{z}$. This won't work out, since **PIOP** enforces prover must abbey the protocol defined, otherwise verifier will catch his malicious behaviors. How? 

<br />

Verifier will challenge (open) these commitments on some random point, this enforces prover provide valid proof, then verifier will check it (*zero-check*). If prover does not know $\mathbf{z}$, then he won't have a interpolated polynomial for it, and he can not evaluate this polynomial with opening point provided by verifier, moreover he can not provide corresponding proof.

<br />

**Open/Verify**

Verifier samples random opening points for these commitments, say $r_{z_A}, r_{z_B}, r_{z_C}, r_{z} \in F_p$. Then prover generate corresponding proofs for them, take $r_{z_A}$ as example, prover derive a quotient polynomial on opening point $r_{z_A}$:

$$
q_{z_A}(X) = \frac{p_{z_A}(X) - p_{z_A}(r_{z_A})}{X - r_{z_A}}
$$

where $p_{z_A}$ is interpolated polynomial for $\mathbf{z_A}$. Prover commits this quotient polynomial with basis $\Sigma_1$:

$$
c_{q_{z_A}} = \langle \Sigma_1, \mathbf{q_{z_A}} \rangle
$$

Similarily applied with $r_{z_B}, r_{z_C}, r_z$:

$$
\begin{aligned}
c_{q_{z_B}} &= \langle \Sigma_1, \mathbf{q_{z_B}} \rangle \\
c_{q_{z_C}} &= \langle \Sigma_1, \mathbf{q_{z_C}} \rangle \\
c_{q_{z}} &= \langle \Sigma_2, \mathbf{q_{z}} \rangle \\
\end{aligned}
$$

<br />

In general, verifier executes a check:

$$
q(X) \overset{?}= \frac{p(X) - p(r)}{X - r}
$$

with bilinear-map:

$$
e(c_{q}, [\tau]_2) \overset{?}= e(c_{p} + r \cdot c_q - [p(r)]_1, [1]_2) 
$$

this check ensures:
1. prover knows the interpolated vector values $\mathbf{p}$
2. prover commit this vector $\mathbf{p}$ with specific domain $K$ which can be $\Sigma_1$ or $\Sigma_2$ in our case (random openning point $r \in \mathbb{F}_p \backslash K$)

<br />

The second check is *consistency check*:

$$
\alpha_A \cdot p_{z_A}(X) + \alpha_B \cdot p_{z_B}(X) + \alpha_C \cdot p_{z_C}(X) \overset{?}= \langle \alpha_A \mathbf{A}(X) + \alpha_B \mathbf{B}(X) + \alpha_C \mathbf{C}(X), \mathbf{z} \rangle
$$

we also utilize bilinear-map:

$$
e(c_{z_A}, [\alpha_A]_2) \cdot e(c_{z_B}, [\alpha_B]_2) \cdot e(c_{z_C}, [\alpha_C]_2) \overset{?}= e(c_z, [1]_2)
$$

<br />

this checks ensures:
1. $\mathbf{z_A}, \mathbf{z_B}, \mathbf{z_C}$ are derived from matrix $\mathbf{A}, \mathbf{B}, \mathbf{C}$ respectively.
2. they share the same witness vector $\mathbf{z}$.

<br />

## Row-Check

In **Lin-Check**, with **PIOP**, prover ensures $\mathbf{z_A}, \mathbf{z_B}, \mathbf{z_C}$ share the same witness vector $\mathbf{z}$. In R1CS, we need to prove the quadratic relation:

$$
\mathbf{z_A} \cdot \mathbf{z_B} \overset{?}= \mathbf{z_C}
$$

With the domain $K$ needed to interpolate above vectors, we can have a **vanishing polynomial**:

$$
v_K(X) = \prod_{i = 0}^{n} (X - k_i), k_i \in K
$$

Then the quotient polynomial:

$$
h(X) = \frac{p_{z_A}(X) \cdot p_{z_B}(X) - p_{z_C}(X)}{v_K(X)}
$$

exists if and only if above quadratic equation holds.

<br />

So, **on prover side**, prover only needs to commit that quotient polynomial $c_{h}$ with $\Sigma_1$. **On verifier side**, checks with bilinear-map:

$$
e(c_{z_A}, c_{z_B}) \overset{?}= e(c_{z_C}, [1]_2) \cdot e(c_h, [v_K(\tau)]_2)
$$

<br />

## PoC Implementaion of Pari

<br />

# DV-KZG Based Pari

<br />

# References

[1] [GARUDA and PARI: Faster and Smaller SNARKs
via Equifficient Polynomial Commitments](https://eprint.iacr.org/2024/1245.pdf)

[2] [PoC implementation of Garuda and Pari](https://github.com/alireza-shirzad/garuda-pari/tree/main)