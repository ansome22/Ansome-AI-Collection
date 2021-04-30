fn randomShuffle(original: any[]) -> any[]{
    list = original.copy();
    n = list.length;
    while n > 1{
        k = random.integerLessThan(n);
        n--;
        list[n], list[k] = list[k], list[n];
    }

    return list
}
