# Background

## Lagrange Interpolation

Assuming there is a domain $K = (k_0, k_1, k_2, ..., k_{n - 1})$, and a vector $\bold{v} = (v_0, v_1, v_2, ..., v_{n - 1})$. We can simulate a curve containing these $n$ points with **Lagrange Interpolation**.

First of all, according the determined domain $\bold{k}$, we can draw $n$ partial lagrange polynomials:
$$
L_i^K(X) = \prod_{j \ne i}^n \frac{X - k_j}{k_i - k_j}, i \in [n]
$$
whose degree is $n - 1$.

Then the lagrange polynomial is actually an inner product:
$$
p(X) = \langle \bold{v}, L^{K}(X) \rangle
$$

<br />

## Naive KZG10

Prover needs to prove to verifier that he knows two vectors $\bold{v_1}, \bold{v_2}$ whose lengths are $D$, with **Lagrange Interpolation**, prover can encode these each vector values to a polynomial with degree $D$, eg. $p_1(X), p_2(X)$.

**Setup**

Given a dimention number $D$, generate:
$$
pp = (G, \tau G, \tau^2 G, ..., \tau^n G)
$$

**Commit**

$$
C_{p_1(X)} = [p_1(X)]_1 = p_1(\tau) G \\
C_{p_2(X)} = [p_2(X)]_2 = p_2(\tau) G \\
$$

sends to verifier.

**Open**

Verifier send a opening $z$ to prover, prover generate proofs.

Firstly, compute quotient polynomials:
$$
q_1(X) = \frac{p_1(X) - p_1(z)}{X - z} \\
q_2(X) = \frac{p_2(X) - p_2(z)}{X - z}
$$
Secondly, commit them:
$$
C_{q_1(X)} = [q_1(X)]_1= q_1(\tau) G \\
C_{q_2(X)} = [q_2(X)]_1 = q_2(\tau) G \\
$$
Lastly, sends $(p_1(z), C_{q_1(X)})$ and $(p_2(z), C_{q_2(X)})$ to verifier.

**Verify**

Verifier checks one-by-one, regarding $p_1(X)$:
$$
p_1(X) - p_1(z) \overset{?}= q_1(X) \cdot (X - z)
$$
verifier checks with bilinear-maps:
$$
e([p_1(X)]_1 - [p_1(z)]_1, [1]_2) \overset{?}= e([q_1(X)]_1, [\tau]_2 - [z]_2)
$$

Similarly applied with $p_2(X)$.

# Naive KZG-based Linear Check

Two equations:
$$
\bold{A} \bold{z} = \bold{z_A} \\
\bold{B} \bold{z} = \bold{z_B} \\
$$
where vector $\bold{z}$ is fixed and unkown for verifier. Prover needs to prove that he owns such a vector satisfy above two equations.

<br />

Assuming a group-based domain $K \subset F_p^{*}$:
$$
K = (1, \omega, \omega^2, ..., \omega^{n - 1})
$$

<br />

**Step 1**:

With **Lagrange Interpolation**, we can encode a vector, for example  $\bold{z_A}$, to a polynomial $p_1(X)$:
$$
p_1: K \longrightarrow \bold{z_A}
$$
it should be like $p_1(X) = p_{10} + p_{11} X + p_{12} X^2 + ... + p_{1{n - 1}} X^{n - 1}$, similar with $p_2(X)$ for vector $\bold{z_B}$:
$$
p_2: K \longrightarrow \bold{z_B}
$$

<br />

**Step 2**:

Then commit these two polynomials (specially for coefficients) with **Pederson Commitment** with basis points $\bold{\Tau} = ([1]_1, [\tau]_1, [\tau^2]_1, ..., [\tau^{n - 1}]_1)$, take $p_1(X)$ for example:
$$
c_{p_1(X)} = \langle \bold{p_1}, \bold{T}\rangle = [p_{10} + p_{11} \tau + p_{12} \tau^2 + ... + p_{1 {n - 1}} \tau^{n - 1}]_1
$$

More importantly, the scalar part of polynomial $p_1(X)$ commitment is an evaluation on $\tau$:
$$
c_{p_1(X)} = [p_1(\tau)]_1
$$
In other words, we can merge **Step 1** and **Step 2** into a single step, but you should notice **Lagrange Interpolation** and **Pederson Commitment** are under the water.

<br />

**Linear Check**

Firstly, we have $n$ partial **Lagrange Polynomial**s, we can directly evaluate them on $\tau$:
$$
L^K(\tau) = (L^K_0(\tau), L^K_1(\tau), ..., L^K_{n - 1}(\tau))
$$

<br />

Secondly, after an inner product we can get the (Pederson) commitment of vector $\bold{z_A}$:
$$
c_{z_A} = [\langle L^K(\tau), \bold{z_A} \rangle]_1
$$

<br />

Thirdly, since $\bold{z_A} = \bold{A} \bold{z}$, we have:
$$
\begin{aligned}
c_{z_A} &= [\langle L^K(\tau), \bold{A} \bold{z} \rangle]_1 \\
&= [\langle L^K(\tau)^T \cdot \bold{A}, \bold{z} \rangle]_1 \\
\end{aligned}
$$


<br />

# KZG-based EPC

Prover needs to prove to verifier that:
$$
\bold{A} \cdot \bold{z} = \bold{p_1} \\
\bold{B} \cdot \bold{z} = \bold{p_2} \\
$$
he knows a secret vector $\bold{z}$ whose matrix multiplication against $\bold{A}, \bold{B}$ are $\bold{p_1}$ and $\bold{p_2}$. Where matrixs $\bold{A}, \bold{B}$ with $n$ columns are known to verifier.

Prover can batch them together with two random factors $\alpha, \beta$:
$$
\alpha \cdot \bold{A} \bold{z} + \beta \cdot \bold{B} \bold{z} = (\alpha \bold{A} + \beta \bold{B}) \bold{z} = \alpha \cdot \bold{p_1} + \beta \cdot \bold{p_2}
$$
where $\alpha \bold{A} + \beta \bold{B}$ is also matrix which is known to verifier at setup time. In this case, with **Lagrange Interpolation**:
- in setup time, prover commits the folded matrix column-by-column.
- in running time, prover commits the matrix multiplication result vector.
- in openning time, verifier sends an openning to prover, then prover sends evaluations and proofs to verifier, and verifier checks.

**Setup**

In general:
$$
Setup(D) \longrightarrow pp
$$

Given a dimention number $D$, generate:
$$
pp = (G, \tau G, \tau^2 G, ..., \tau^n G)
$$

**Specialize**

In general:
$$
Specialize(pp, \bold{A}, \bold{B}) \longrightarrow (ck, vk)
$$
where $ck$ is commitment key for commit purpose, and $vk$ is verification key for verify purpose. $\bold{A}$ and $\bold{B}$ are two matrixs with $n$ columns.

Prover needs to commit these two matrixs and batch them together with two random factors $\alpha, \beta$:
$$
\begin{rcases}
C_{\bold{A}} = \{a_i(\tau)G\}_{i = 1}^n \\
C_{\bold{B}} = \{b_i(\tau)G\}_{i = 1}^n \\
\end{rcases} 
\longrightarrow
\begin{aligned}
\alpha C_{\bold{A}} + \beta C_{\bold{B}} &= \{(\alpha \cdot a_i(\tau) + \beta \cdot b_i(\tau))G\}_{i = 1}^n \\
&= \{\alpha \cdot [a_i(X)]_1 + \beta \cdot [b_i(X)]_1 \}_{i = 1}^n \\
\end{aligned}
$$
along with $pp$ constituting the $ck$:
$$
\begin{aligned}
ck &= (pp, \alpha C_{\bold{A}} + \beta C_{\bold{B}}) \\
&= ((G, \tau G, \tau^2 G, ..., \tau^n G), \{\alpha \cdot [a_i(X)]_1 + \beta \cdot [b_i(X)]_1 \}_{i = 1}^n) \\
\end{aligned}
$$

In the meantime, the $vk$ is:
$$
vk = ([\tau]_2, [\alpha]_2, [\beta]_2)
$$

Note that, $ck$ are all EC points on $G_1$, and $vk$ are all EC points on $G_2$.

**Commit**



**Verify**
