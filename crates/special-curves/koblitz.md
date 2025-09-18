- [Background](#background)
  - [Eigen Value](#eigen-value)
  - [Trace](#trace)
  - [Trace of Frobenius](#trace-of-frobenius)
  - [A Few Numbers](#a-few-numbers)

# Background

## Eigen Value

Assuming there is a matrix $\tau$:

$$
\tau = \begin{pmatrix} a & b \\ c & d \end{pmatrix}
$$

its **eigen value** $\lambda$ satisfies a relation:

$$
\mathbf{v} \cdot (\tau - \lambda \mathbf{I}) = 0
$$

the solution $\mathbf{v}$ exists if and only if $\det(\tau - \lambda \mathbf{I}) = 0$, that’s to say:

$$
\begin{aligned}|\begin{pmatrix} a - \lambda & b \\ c & d - \lambda \end{pmatrix}| &= (a - \lambda)(d - \lambda) - bc \\ &= \lambda^2 - (a + d)\lambda + ad - bc \\ &= \lambda^2 - Tr(\tau) \cdot \lambda + det(\tau) \\ &= 0 \end{aligned}
$$

So we have conclusions about the relation between **eigen values** and **trace, determint** of matrix:

$$
\lambda_1 + \lambda_2 = Tr(\tau) \\ \lambda_1 \cdot \lambda_2 = Det(\tau) \\
$$

## Trace

As we know, matrix can be regarded as linear map, say:

$$
\tau = \begin{pmatrix} a & b \\ c & d \end{pmatrix}
$$

the **eigen values** of $\tau$ is $(\lambda_1^{(1)}, \lambda_2^{(1)})$, and the **trace value** $Tr(\tau) = \lambda_1^{(1)} + \lambda_2^{(1)} = a + d$.

$\tau^2$ means apply matrix two times:

$$
\tau^2 = \begin{pmatrix} a^2 + bc & ab + bd \\ ac + cd & bc + d^2 \end{pmatrix}
$$

the **trace value**

$$
\begin{aligned} Tr(\tau^2) &= \lambda_1^{(2)} + \lambda_2^{(2)} \\ &= a^2 + 2bc + d^2 \\ &= (\lambda_1^{(1)} + \lambda_2^{(1)})^2 - 2 \cdot \lambda_1^{(1)} \lambda_2^{(1)} \\ &= {\lambda_1^{(1)}}^2 + {\lambda_2^{(1)}}^2 \end{aligned}
$$

In general, $\tau^d$ means apply matrix $d$ times, the **trace value** $Tr(\tau^d) = \lambda_1^{(d)} + \lambda_2^{(d)} = {\lambda_1^{(1)}}^d + {\lambda_2^{(1)}}^d.$

## Trace of Frobenius

$N(X)$ calls the norm $X$, where $X$ is a quadratic (complex) field. That’s to say:

$$
N(X) = X \cdot \overline{X}
$$

So, we have:

$$
N(1 - \lambda^d) = (1 - \lambda^d) \cdot (1 - \overline{\lambda ^d}) = 1 - (\lambda ^d + \overline{\lambda ^d}) + \lambda ^d \cdot \overline{\lambda ^d}
$$

since $\lambda, \overline{\lambda}$ are two complex roots of the characteristic polynomial of **Frobenius endomorphism** $T$ :

$$
\chi(T) = T^2 - \mu T + 2
$$

that’s to say, $\lambda + \overline{\lambda} = \mu, \lambda \cdot \overline{\lambda} = 2$. Actually we call these two roots the **eigen values** of $T$ which is a kind of linear operator and can be represented as a map matrix, and its determination is a constant 2. This is a special case for $\mathbb{F}_q = \mathbb{F}_{p^d}$ where characteristic  $p = 2$ .

*Note that, the number of eigen values depends on the **rank** of this matrix $T$ .*

In general, regarding $T^d$ we have:

$$
Tr(T^d) = \lambda^d + \overline{\lambda^d} = \lambda^d + \overline{\lambda}^d = \mu \\ Det(T^d) = \lambda^d \cdot \overline{\lambda^d} = (\lambda \cdot \overline{\lambda})^d = 2^d
$$

where $\lambda^d, \overline{\lambda^d}$ are two **eigen value** s of  $T^d$, and $\lambda, \overline{\lambda}$ are two **eigen value** s of $T$.  Therefore we have:

$$
N(1 - \lambda^d) = 1 - Tr(T^d) + Det(T^d) = |E(\mathbb{F}_{2^d})|
$$

where $Det(T^d) = 2^d$.

## A Few Numbers

- $2^d \cdot 2^d = 2^{2d}$ is the number of pair $(x, y) \in \mathbb{F}_{2^d}^2$
- $|E_{\mathbb{F}_{2^d}}| = N(\lambda^d - 1)$ is the number of points $P = (x, y)$ on special elliptic curve $E_{\mathbb{F}_{2^d}}: y^2 + xy = x^3 + a_2 x^2 + 1$.
- $|Q|$ is the order of point $Q \in E_{\mathbb{F}_{2^d}}$, the maximum number is $|E_{\mathbb{F}_{2^d}}| = N(\lambda^d - 1)$.
- $N(\delta) = N(\frac{\lambda^d - 1}{\lambda - 1}) = \frac{N(\lambda^d - 1)}{N(\lambda - 1)}$ is the order of some point $P = [2] Q \in E_{\mathbb{F}_{2^d}}$ where $|Q| = |E_{\mathbb{F}_{2^d}}| = N(\lambda^d - 1)$ as we have $N(\lambda - 1) = (\lambda - 1) \cdot (\bar{\lambda} - 1) = Det(T) - Tr(T) + 1$
    - if $a_2 = 1$, then $N(\lambda - 1) = 2$
    - if $a_2 = 0$, then $N(\lambda - 1) = 4$

Therefore, we have the following relationship among all these number:

$$
2^{2d} \gt |E_{\mathbb{F}_{2^d}}| \ge |Q| \gt |P|
$$

Observing that $\delta = \frac{\lambda^d - 1}{\lambda - 1}$, and $N(\delta)$ is usually a prime number for application consideration. For example:

$$
E_{\mathbb{F}_{2^{11}}}: y^2 + xy = x^3 + x^2 + 1 
$$

then $|E_{\mathbb{F}_{2^{11}}}| = 2 \cdot 991$, where 991 is just a prime number.