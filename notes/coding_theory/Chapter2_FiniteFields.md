# Definitions

- **Injective** (one-to-one). different inputs maps to different outputs.

- **Sujective** (onto). every output has corresponding input.

- **Bijective** (one-to-one and onto). every output has unique corresponding input.

- **Homomorphism**. Known as **General Structure-Preserving Map**.
  
  A map between two groups, only preserves the operation of the structures. i.e. $f(x \times y) = f(x) \times f(y)$. The properties are:
  
  - No injective or sujective. In this case may loss information, since two elements of group $A$ may map on element of group $B$ (non-injective), or some element of group $B$ can not reach out to group $A$ (non-surjective).
  
  - Kernel. The elements maps to the identity of group $B$ might be non-trivial.
  
  For example, $f: (\Z, +)  \rightarrow (\Z_5, +)$ defined by $f(x) = x \mod 5$ is a homomorphism, operation is preserved $(x + y) \mod 5 = (x \mod 5) + (y \mod 5)$, but it does not satisfy injective, $f(0) = f(5) = 0$.

- **Isomorphism**. Known as **Perfect Structure-Preserving Equivalence**.
  
  A map between two groups, not only preserves preserves the operation of the structures, but also satisfy **bijective**. It's a bijective homomorphism. The properties are:
  
  - No information loss in the map $f: A \rightarrow B$.
  
  - Inverse $f^{-1}: B \rightarrow A$  is also a homomorphism. 
  
  - Structure identity $A \cong B$ (A is congruent with B), they are algebraically identical, only differs in labeling.
  
  For example, $f: (\R, +)  \rightarrow (\R, *)$ defined by $f(x) = e^x$ is a isomorphism, operation is preserved $e^{x + y} = e^x * e^y$, and it satisfy bijective. 



# Background

- **Group**  is defined upon only multiplication.

- **Ring** is defined upon both addition and multiplication.
  
  - division can be possible or not

- **Field** is a special **Ring** , satisfying multplicative inverse.
  
  - substraction is a special addition in general, $a - b = a + neg(b)$.
  
  - division is also a special multiplication in general, and multplicative inverse make division true for field, $\frac{a}{b} = a \cdot inv(b)$.
