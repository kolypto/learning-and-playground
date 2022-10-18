# cython: profile=True
# cython: linetrace=True



import cython


@cython.ccall
def primes(nb_primes: cython.int):
    # Max limit on primes
    if nb_primes > 1000:
        nb_primes = 1000

    # Initialize array. When compiled, fill with zeroes
    # Such array is allocated on C call stack. This is bad: learn how to use arrays properly!
    p: cython.int[1000]
    if not cython.compiled:
        p = [0] * 1000
    
    # Give types to all variables
    i: cython.int 
    len_p: cython.int = 0 
    n: cython.int = 2

    while len_p < nb_primes:
        for i in p[:len_p]:
            if n % i == 0:
                break
        else:
            p[len_p] = n
            len_p += 1
        
        n += 1

    result_as_list = [prime for prime in p[:len_p]]
    return result_as_list