## BitCommitment

**Goal**: ensure all input keys for some wire bit, say $b_i = 1$, across all copies GC are consistent, they are bound to a specific bitcommitment.

![](/Users/yuanpingzhou/Project/workspace/zkp/babylon/bitvm3_notes/lego-Page-3.drawio.png)



#### Notation

- permutation bit $\pi = 0$ indicates no order changes, $[a, b] \overset{\pi = 0}\longrightarrow [a, b]$, otherwise swap them $[a, b] \overset{\pi = 1}\longrightarrow [b, a]$.

- bitcommitment is a square operation $[a] = a^2$, it is not computational inversible.

#### Commit

For each of garbler's input wire:

- BitCommitment for target bit $b_i = 1$ (evaluation bit)
  
  - Garbler encode $b_i = 1$ into a group element $u_i \leftarrow b_i$ explicitly, which is privacy for evaluator while evaluating.
  
  - Garbler send bitcommitment of $u_i$, say $[u_i] = u_i^2$, to evaluator.

- BitCommitment for permutated target bit
  
  - Garbler randomly sample a permutation bit $\pi_i = 0$.
  
  - Garbler encode $\pi_i = 0$ into a group element $\alpha_i \leftarrow \pi_i$ explicitly.
  
  - Garbler calculate the permutated $v_i = u_i \otimes \alpha_i$.
  
  - Garbler send bitcommitment of $v_i$, say $[v_i] = v_i^2$, to evaluator.

- Commit input pair keys with permutated order $\pi_i = 0$
  
  - Garbler computes commitments for input keys $[K_i^{0}, K_i^{1}] \longrightarrow [H(K_i^{0}), H(K_i^{1})]$.
  
  - Garbler applies permuated order $\pi_i = 0$ on commitments $[H(K_i^{0}), H(K_i^{1})] \overset{\pi_i = 0}\longrightarrow [H(K_i^{0}), H(K_i^{1})].$ 
  
  - Garbler send $H_i = [H(K_i^{0}), H(K_i^{1})]$ to evaluator. 

#### Reveal for Verification

Garbler reveals:

- group element $\alpha_i$ which encodes permutation bit $\pi_i = 0$ explicitly.

- input pair keys $K_i^{0}, K_i^{1}$ with permutated order $\pi_i = 0$.

Evaluator checks:

- consistency of two bitcommitments with permutation bit, $[u_i] \overset{\pi_i}\longrightarrow [v_i]$ .
  
  $$
  [u_i] \otimes [\alpha_i] = u_i^2 \otimes \alpha_i^2 \overset{?}= [v_i] = (u_i \otimes \alpha_i)^2
  $$

- correctness of input key commitments
  
  $$
  [K_i^0, K_i^1] \longrightarrow [H(K_i^0), H(K_i^1)] \overset{\pi_i = 0}\longrightarrow [H(K_i^0), H(K_i^1)] \overset{?}= H_i 
  $$

#### Reveal for Evaluation

Garbler reveals:

- group element $v_i$ which encodes permutated target bit. NOTE that the permuated target bit indicates in which position the revealed input key stands.

- input key $K_i^{b_i}$ bind with target bit $b_i$.

Evaluator checks:

- correctness of bitcommitment for permutated target bit
  
  $$
  v_i^2 \overset{?}= [v_i]
  $$

- correctness of target input key
  
  $$
  H_i[b_i'] \overset{?}= H(K_i^{b_i})
  $$
  
  where $b_i'$ is the permuated target bit (or index) $b_i' = b_i \otimes \pi_i$, and evaluator can easily extract it from revealed group element $v_i$ since $b_i'$ is encoded in it explicitly.



#### Properties of BitCommitment

- $b_i' = b_i \otimes \pi_i$, all are bit values.

- $v_i = u_i \otimes \alpha_i$, where group elements $v_i, u_i, \alpha_i$ all encodes bit values $b_i', b_i, \pi_i$.

- $[v_i] = [u_i] \otimes [\alpha_i]$, where bitcommitments are for $v_i, u_i, \alpha_i$ individually.


