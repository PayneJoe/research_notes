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

> If $a$ and $n$ are co-prime numbers, then we must have $a^{\phi(n)} \equiv 1 (\mod n)$, where $\phi(n) = \phi(p) \cdot \phi(q) = (p - 1) \cdot (q - 1)$, $p$ and $q$ are two prime factors of $n$.

Proof:

All co-prime numbers of $n$ forms a multiplicative group which is isomorphism with $\mathbb{F}_{\phi(n)}^{\times}$. 

According to Lagrange Theorem, the size of any subgroup of group $\mathbb{F}_{\phi(n)}^{\times}$ is a factor of $\phi(n)$, say $|a| = m, m \mid \phi(n)$.

So we must have $a^m \equiv a^{\phi(n)} \equiv 1 (\mod n)$.

<br />

There is a problem that if $a$ is not co-prime with $n$, how about that?

#### Carmichael Function

> If $a$ is any number, then we have $a^{\lambda(n)} \equiv 1 (\mod n)$, where $\lambda(n) = lcm(p - 1, q - 1)$, and $p$ and $q$ are two prime factors of $n$.

Proof:

Observing that $lcm(p - 1, q - 1)$ is a multiple of $p - 1$ or $q - 1$.

Case 1: If $a$ is a coprime of $n$, then it must coprime with $p$ and $q$. We have:
- $a^{p - 1} \equiv 1 (\mod p)$
- $a^{q - 1} \equiv 1 (\mod q)$

According to **Chinese Reminder Theorem**, so we have $a^{\lambda(n)} = a^{lcm(p - 1, q - 1)} \equiv 1 (\mod n)$.

Case 2: If $a$ is not a coprime of $n$, then is must be a multiple of $p$ or $q$. Assume $a = k \cdot p$, then we have:
- $a^{p - 1} \equiv 0 (\mod p) \equiv a (\mod p)$.
- $a^{q - 1} \equiv 1 (\mod q)$.

According to **Chinese Reminder Theorem**, we have $a^{\lambda(n)} = a^{lcm(p - 1, q - 1)} \equiv 1 (\mod n)$.

<br />

In conclusion, **Carmichael Function** is a more strict function against **Euler Totient Function**.

## RSA Algorithm

#### Setup

$n$ is a large number which can be factorised into two prime factors, $p$ and $q$. And an euler number $\phi(n) = (p - 1) \cdot (q - 1)$, public and private keys are:
- public key is $(n, e)$, $e$ is a random nunber.
- secret key is $(n, d)$, $d \equiv e^{-1} (\mod \phi(n))$.

#### Encoding

Assuming message is a integer $m$:

$$
C \equiv m^e (\mod n)
$$

#### Decoding

$$
m \equiv C^d (\mod n)
$$

#### Why Works

$$
m = C^d = (m^e)^d \equiv m^{e \cdot d} (\mod n)
$$
According to **Euler Totient Function**, it holds if and only if $e \cdot d = \phi(n) + 1$, assuming that $(m, n) = 1$ (two coprime numbers).

#### Security Analysis

RSA security depends on the **hard problem of prime factoring large number**. More specially, knowing large number $n$ we can find its pair coprime factors $p$ and $q$. As a result, we do not have the **Euler** number $\phi(n)$. Therefore, given public number $e$, we can derive its secret number $d \equiv e^{-1} (\mod \phi(n))$.

You may also notice that there is still a assumption, that is $(m, n) = 1$. So what to do if $(m, n) \ne 1$? Actually the probability of $(m, n) \ne 1$ is negligible in general.


And another problem is why RSA use **Euler** function, not **Carmichael** function, the later is more strict than the former one though? There are three considerations:
-  a historical reason exists, **Euler** function is much older than **Carmichael**.
- computing $\phi(n) = (p - 1) \cdot (q - 1)$ is much easier than computing $lcm(p - 1, q - 1)$, since the later on involves $GCD$s, it's more computational.
- there is no security benefits to replace **Euler** with **Carmichael**. 