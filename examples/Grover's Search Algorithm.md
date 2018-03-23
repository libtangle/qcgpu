# Grover's Search Algorithm

Grover's search algorithm is an algorithm for quantum computers. The algorithm finds, with high probability the unique input to an oracle (black box) function, in O($$\sqrt{N}$$) operations, where $$N$$ is the number of items beng searched for.

The same problem cannot be solved in fewer then O($$N$$) operations. This quadratic sppedup was shown to also be optimal.



With $$n$$ qubits, we can search a $$N$$ values.
Starting with the $$n$$ qubits, initialize all to a state of:

$$
\lvert 0 \rangle ^{\otimes n}
$$

Now all of the qubits need to be in a state where the probability of getting their value when measured in equal. This is done by applying hadamard gates to each qubit, $$H^{\otimes n}$$, which requires $$O(\lg N) = O(\lg 2^n) = O(n)$$ operations. 

$$
\lvert\Psi\rangle = H^{\otimes n} \lvert 0 \rangle ^{\otimes n} = \frac{1}{\sqrt{2^n}} \sum_{x = 0}^{2^n - 1} \lvert x\rangle
$$

We need a way to provide the quantum computer to check if it is the correct
item. We need to encode the list, in terms of a function $$f$$ which returns
$$f(x) = 1$$ for the item we are looking for, and $$f(x) = 0$$ for all other items.

This function must be encoded into a unitary matrix, so it can be used on the
quantum computer. It is commonly called an oracle.
We choose a binary encoding of the items $$x,w \in {0, 1}^n$$; where $$w$$ is the
item we are looking for. We then define an oracle matrix $$U_f$$ to act on any standard basis
$$\lvert x \rangle$$ by: 
$$
U_f \lvert x \rangle = (-1)^{f(x)} \lvert x \rangle
$$
The oracle implementation will often use an extra scratch or ancillary qubit.