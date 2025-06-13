## Mathematical Background

#### Euler Totient Function

$\phi(n)$ refers the total number of co-prime numbers with the range $[1, n)$, where $n \in \mathbb{Z}$.

There are a few good properties:
- **Multiplicity**, if two numbers $a$ and $b$ are co-prime, then:
    $$
        \phi(a \cdot b) = \phi(a) \cdot \phi(b)
    $$

- **Prime Power**, for a prime number $p$ and exponent $k > 1$, then:
  $$
    \phi(p^k) = p^k - p^{k - 1} = p^k \cdot (1 - \frac{1}{p})
  $$

- **Prime Number**, for a prime numnber $p$, then:
    $$
        \phi(p) = p - 1
    $$

<br />

More importantly, what we called **Euler's Theorem**:

if $a$ and $n$ are co-prime numbers, then we must have $a^{\phi(n)} \equiv 1 (\mod n)$.

Proof:

All co-prime numbers of $n$ forms a multiplicative group which is isomorphism with $\mathbb{F}_{\phi(n)}^{\times}$. 

According to Lagrange Theorem, the size of any subgroup of group $\mathbb{F}_{\phi(n)}^{\times}$ is a factor of $\phi(n)$, say $|a| = m, m \mid \phi(n)$.

So we must have $a^m \equiv a^{\phi(n)} \equiv 1 (\mod n)$.