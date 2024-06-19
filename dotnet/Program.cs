using System;
using System.Diagnostics;

public class QuickSortExample
{
    public static void Main()
    {
        int[] arr = { 34, 7, 23, 32, 5, 62, 1, 31, 32, 5 };
        // var segment = new ArraySegment<int>(arr);
        // clear(segment, arr.Length - 1);
        // Console.WriteLine("arr: " + String.Join(", ", arr));

        Console.WriteLine("Original array: " + String.Join(", ", arr));

        var start = Stopwatch.StartNew();
        QuickSort(arr, 0, arr.Length - 1);
        var duration = start.Elapsed;
        Console.WriteLine("Time taken: " + duration);

        Console.WriteLine("Sorted array: " + String.Join(", ", arr));
    }

    // static void clear(ArraySegment<int> segment, int deep)
    // {
    //     if (deep >= 0)
    //     {
    //         segment[deep] = 0;
    //         segment = segment[0..deep];
    //         // list = list.GetRange(0, deep);
    //         // clear(list, deep - 1);
    //         clear(segment, deep - 1);
    //     }
    // }

    public static void QuickSort<T>(T[] arr, int low, int high) where T : IComparable<T>
    {
        if (low < high)
        {
            int pivotIndex = Partition(arr, low, high);
            QuickSort(arr, low, pivotIndex - 1);
            QuickSort(arr, pivotIndex + 1, high);
        }
    }

    public static int Partition<T>(T[] arr, int low, int high) where T : IComparable<T>
    {
        T pivot = arr[high];
        int i = low - 1;
        for (int j = low; j < high; j++)
        {
            if (arr[j].CompareTo(pivot) < 0)
            {
                i++;
                Swap(ref arr[i], ref arr[j]);
            }
        }
        Swap(ref arr[i + 1], ref arr[high]);
        return i + 1;
    }

    private static void Swap<T>(ref T a, ref T b)
    {
        T temp = a;
        a = b;
        b = temp;
    }
}