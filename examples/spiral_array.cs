var sizes = new (int, int, int)[] { (9, 3, 3), (16, 4, 4), (25, 5, 5), (24, 4, 6), (18, 6, 3) };
// var sizes = new (int, int, int)[] { (9, 3, 3) };
foreach (var size in sizes)
{
    Console.WriteLine("{0}", size);

    var arr = SpiralArray(size.Item1, size.Item2, size.Item3);

    for (int i = 0; i < size.Item2; i++)
    {
        for (int j = 0; j < size.Item3; j++)
        {
            Console.Write("{0,5}", arr[i, j]);
        }
        Console.WriteLine();
    }
}

int[,] SpiralArray(int len, int rows, int cols)
{
    var arr = new int[rows, cols];
    var (top, bottom, left, right) = (0, rows - 1, 0, cols - 1);
    Console.WriteLine("{0}", (top, bottom, left, right));
    var num = 1;
    while (top <= bottom && left <= right)
    {
        foreach (var i in Enumerable.Range(left, right - left + 1))
        {
            arr[top, i] = num;
            num += 1;
        }
        top += 1;
        foreach (var i in Enumerable.Range(top, bottom - top + 1))
        {
            arr[i, right] = num;
            num += 1;
        }
        right -= 1;
        if (top <= bottom)
        {
            foreach (var i in Enumerable.Range(left, right - left + 1).Reverse())
            {
                arr[bottom, i] = num;
                num += 1;
            }
            bottom -= 1;
        }
        if (left <= right)
        {
            foreach (var i in Enumerable.Range(top, bottom - top + 1).Reverse())
            {
                arr[i, left] = num;
                num += 1;
            }
            left += 1;
        }
    }
    return arr;
}
