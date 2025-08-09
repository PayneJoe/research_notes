# Naitve KZG

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

# KZG-based EPC

**Setup**

Given a dimention number $D$, generate:
$$
pp = (G, \tau G, \tau^2 G, ..., \tau^n G)
$$

**Specification**

**Commit**

**Verify**
