## Determint of Matrix

$$
det(U) = |U|
$$

Determint is a scalar of a matrix, encoding great properties:

- invertibility
    
    If $U$ is not invertible, then $det(U) = 0$. So determint can be an indicator for the invertibility of a matrix.
- row operations
    - swap row, determint sign reversed
    - scaling one row, determint also scaled

- transpose
    $$
        det(U) = det(U^T)
    $$

## Full Rank Lattice

Full rank lattice means the number of basis of a lattice equals the dimension of a basis space.

$$
\begin{pmatrix}
    b_{00} & b_{01} & b_{02} \\
    b_{10} & b_{11} & b_{12} \\
    b_{20} & b_{21} & b_{22} \\
\end{pmatrix}
$$

## Unimodular Matrix

The determint of a matrix is constant and fixed:

$$
det(U) =\pm 1
$$

## Determint of Lattice

$$
det(\varLambda) = \sqrt{det(\bold{B}^T \bold{B})}
$$

If $\varLambda$ is a full rank lattice, then $det(\varLambda) = |det(\bold{B})|$, the determint of lattice is fixed, not dependent on specific set of basis.