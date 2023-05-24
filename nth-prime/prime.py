def isPrime(k):

    # Corner cases
    if (k <= 1):
        return 0
    if (k == 2 or k == 3):
        return 1

    # below 5 there is only two prime numbers 2 and 3
    if (k % 2 == 0 or k % 3 == 0):
        return 0

  # Using concept of prime number can be represented in form of (6*k + 1) or(6*k - 1)
    end = 1 + int(k ** 0.5)
    for i in range(5, end, 6):
        if (k % i == 0 or k % (i + 2) == 0):
            return 0

    return 1

print("95 is prime", isPrime(95))
