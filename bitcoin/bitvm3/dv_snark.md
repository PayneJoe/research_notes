# Notations
- $n$ denotes the number of columns of a R1CS matrix, also the length of witness vector.

<br />

# Background

## Lagrange Interpolation

Assuming there is a domain $K = (k_0, k_1, k_2, ..., k_{n - 1})$, and a vector $\bold{y} = (y_0, y_1, y_2, ..., y_{n - 1})$. We can simulate a curve containing these $n$ points with **Lagrange Interpolation**.

First of all, according the determined domain $\bold{k}$, we can draw $n$ partial lagrange polynomials:
$$
L_i^K(X) = \prod_{j \ne i}^n \frac{X - k_j}{k_i - k_j}, i \in [n]
$$
whose degree is $n - 1$.

Then the lagrange polynomial is actually an inner product:
$$
p(X) = \langle L^{K}(X), \bold{y} \rangle
$$

<br />

## Pederson Commitment

Assuming there is a basis vector, say:
$$
\bold{b} = ([b_0]_1, [b_1]_1, [b_2]_1, ..., [b_{n - 1}]_1)
$$

We want to commit a polynomial whose coefficient vector is $\bold{x}$, then:
$$
c_{x} = \langle \bold{x}, \bold{b} \rangle
$$

In general, the elements of basis **Pederson Commitment** is discrete and random. But if the basis $\bold{b}$ is algebraic, say:
$$
\bold{b} = ([1]_1, [\tau]_1, [\tau^2]_1, ..., [\tau^{n - 1}]_1)
$$
then the **Pederson Commitment** for target polynomial would be:
$$
c_{p(X)} = [p(\tau)]_1 = [\langle L^K(\tau), \bold{y} \rangle]_1
$$

<br />

# DV-KZG

# Pari

In R1CS, there are two checks, **Lin-check** and **Row-check**.

## Lin-Check

$$
\bold{A} \bold{z} \overset{?}= \bold{z_A} \\
\bold{B} \bold{z} \overset{?}= \bold{z_B} \\
\bold{C} \bold{z} \overset{?}= \bold{z_C} \\
$$
With three random factors $\alpha, \beta, \gamma$, we can batch them together:
$$
(\alpha \cdot \bold{A} + \beta \cdot \bold{B} + \gamma \cdot \bold{C}) \bold{z} = \langle \alpha \cdot \bold{A} + \beta \cdot \bold{B} + \gamma \cdot \bold{C}, \bold{z} \rangle \overset{?}= \alpha \cdot \bold{z_A} + \beta \cdot \bold{z_B} + \gamma \cdot \bold{z_C}
$$

<br />

#### Setup

In setup phase, we need to prepare a new basis for committing witness vector $\bold{z}$:
$$
m_i(\tau) = \alpha \cdot \langle L^K(\tau), \bold{a_i} \rangle + \beta \cdot \langle L^K(\tau), \bold{b_i} \rangle + \gamma \cdot \langle L^K(\tau), \bold{c_i} \rangle, i \in [n] 
$$
where $\bold{a_i}, \bold{b_i}, \bold{c_i}$ are $i$-th columns of R1CS matrix $\bold{A}, \bold{B}, \bold{C}$ respectively, (Pederson) committing and batching them together constitutes a new basis $\bold{\Sigma_2}  = ([m_0(\tau)]_1, [m_1(\tau)]_1, [m_2(\tau)]_1, ..., [m_{n - 1}(\tau)]_1)$.

<br />

Along with lagrange basis $\bold{\Sigma_1} = ([L_0^K(\tau)]_1, [L_1^K(\tau)]_1, [L_2^K(\tau)]_1, ..., [L_{n - 1}^K(\tau)]_1)$ we have the commitment key specially for the purpose of commitments:
$$
ck = (\bold{\Sigma_1}, \bold{\Sigma_2})
$$

<br />

#### Run-time

**Commit Phase**

Firstly, prover needs to (Pederson) commit the right-hand target vectors $\bold{z_A}, \bold{z_B}, \bold{z_C}$ with basis $\Sigma_1$:

$$
c_{z_A} = \langle \bold{\Sigma_1}, \bold{z_A} \rangle \\
c_{z_B} = \langle \bold{\Sigma_1}, \bold{z_B} \rangle \\
c_{z_C} = \langle \bold{\Sigma_1}, \bold{z_C} \rangle \\
$$

<br />

Secondly, prover needs to (Pederson) commit the witness vector $z$ with basis $\Sigma_2$:
$$
c_z = \langle \bold{\Sigma_2}, \bold{z} \rangle
$$

<br />

One question about $c_z$, what if prover does not commit $\bold{z}$ with $\Sigma_2$? If he just fold $c_{z_A}, c_{z_B}, c_{z_C}$ with that three random $\alpha$s, since we all know that:
$$
c_z = \alpha_A \cdot c_{z_A} + \alpha_B \cdot c_{z_B} + \alpha_C \cdot c_{z_C}
$$
Actually this implies that, the prover does not need to without know witness $\bold{z}$. This won't work out, since **PIOP** enforces prover must abbey the protocol defined, otherwise verifier will catch his malicious behaviors. How? 

<br />

Verifier will challenge (open) these commitments on some random point, this enforces prover provide valid proof, then verifier will check it (*zero-check*). If prover does not know $\bold{z}$, then he won't have a interpolated polynomial for it, and he can not evaluate this polynomial with opening point provided by verifier, moreover he can not provide corresponding proof.

<br />

**Open/Verify**

Verifier samples random opening points for these commitments, say $r_{z_A}, r_{z_B}, r_{z_C}, r_{z} \in F_p$. Then prover generate corresponding proofs for them, take $r_{z_A}$ as example, prover derive a quotient polynomial on opening point $r_{z_A}$:
$$
q_{z_A}(X) = \frac{p_{z_A}(X) - p_{z_A}(r_{z_A})}{X - r_{z_A}}
$$
where $p_{z_A}$ is interpolated polynomial for $\bold{z_A}$. Prover commits this quotient polynomial with basis $\Sigma_1$:
$$
c_{q_{z_A}} = \langle \Sigma_1, \bold{q_{z_A}} \rangle
$$
Similarily applied with $r_{z_B}, r_{z_C}, r_z$:
$$
\begin{aligned}
c_{q_{z_B}} &= \langle \Sigma_1, \bold{q_{z_B}} \rangle \\
c_{q_{z_C}} &= \langle \Sigma_1, \bold{q_{z_C}} \rangle \\
c_{q_{z}} &= \langle \Sigma_2, \bold{q_{z}} \rangle \\
\end{aligned}
$$

<br />

In general, verifier executes this *zero-check*:
$$
q(X) \overset{?}= \frac{p(X) - p(r)}{X - r}
$$
with bilinear-map:
$$
e(c_{q}, [\tau]_2) \overset{?}= e(c_{p} - [p(r)]_1, [1]_2) \cdot e(r \cdot c_q, [1]_2)
$$
this check ensures:
1. prover knows the interpolated vector values $\bold{p}$
2. prover commit this vector $\bold{p}$ with specific domain $K$ which can be $\Sigma_1$ or $\Sigma_2$ in our case (random openning point $r \in \mathbb{F}_p \backslash K$)

<br />

The second check is *consistency check*:
$$
\alpha_A \cdot p_{z_A}(X) + \alpha_B \cdot p_{z_B}(X) + \alpha_C \cdot p_{z_C}(X) \overset{?}= \langle \alpha_A \bold{A}(X) + \alpha_B \bold{B}(X) + \alpha_C \bold{C}(X), \bold{z} \rangle
$$
we also utilize bilinear-map:
$$
e(c_{z_A}, [\alpha_A]_2) \cdot e(c_{z_B}, [\alpha_B]_2) \cdot e(c_{z_C}, [\alpha_C]_2) \overset{?}= e(c_z, [1]_2)
$$

<br />

## Row-Check

In **Lin-Check**, with **PIOP**, prover ensures $\bold{c_z}$ is the commitment with  

$$
\bold{z_A} \cdot \bold{z_B} \overset{?}= \bold{z_C}
$$

# References