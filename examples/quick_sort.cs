int[] arr = { 34, 7, 23, 32, 5, 62, 1, 31, 32, 5 };
Console.WriteLine("{0,-18} {1}", "Original array: ", String.Join(", ", arr));

var segment = new ArraySegment<int>(arr);
var start = Stopwatch.StartNew();
QuickSort(segment);
var duration = start.Elapsed;
Console.WriteLine("Time taken: " + duration);

Console.WriteLine("{0,-18} {1}", "Sorted array: ", String.Join(", ", arr));

void QuickSort<T>(ArraySegment<T> segment) where T : IComparable<T>
{
    if (segment.Count > 1)
    {
        var pivotIndex = Partition(segment);
        // 跳过已经排序好的索引
        var left = segment[0..pivotIndex];
        var right = segment[(pivotIndex + 1)..];

        // Console.WriteLine("Left array: " + String.Join(", ", left));
        // Console.WriteLine("Right array: " + String.Join(", ", right));

        QuickSort(left);
        QuickSort(right);
    }
}

int Partition<T>(ArraySegment<T> segment) where T : IComparable<T>
{
    var pivotIndex = segment.Count / 2;
    var lastIndex = segment.Count - 1;
    //将中间值移动到尾部
    Swap(segment, pivotIndex, lastIndex);

    var storeIndex = 0;
    for (var index = 0; index < lastIndex; index++)
    {
        if (segment[index].CompareTo(segment[lastIndex]) < 0)
        {
            Swap(segment, index, storeIndex);
            storeIndex++;
        }
    }

    //将中间值从尾部移动到排序后的位置
    Swap(segment, storeIndex, lastIndex);
    return storeIndex;
}

void Swap<T>(ArraySegment<T> segment, int i, int j)
{
    var tmp = segment[i];
    segment[i] = segment[j];
    segment[j] = tmp;
}