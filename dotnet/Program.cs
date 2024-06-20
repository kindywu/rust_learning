using System;
using System.Diagnostics;

public class QuickSortExample
{
    public static void Main()
    {
        var arr = new int[] { 34, 7, 23, 32, 5, 62, 1, 31, 32, 5 };
        // var arr = new int[] { 2, 3, 4, 1 };
        // var arr = new int[] { 2, 2, 3, 3, 4, 4, 1, 1, 0, 0 };

        Console.WriteLine("Original array: " + String.Join(", ", arr));

        var start = Stopwatch.StartNew();
        QuickSort(arr, 0, arr.Length - 1);
        var duration = start.Elapsed;
        Console.WriteLine("Time taken: " + duration);

        Console.WriteLine("Sorted array: " + String.Join(", ", arr));
    }

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
        int i = low;
        for (int j = low; j < high; j++)
        {
            if (arr[j].CompareTo(pivot) < 0)
            {
                Swap(ref arr[i], ref arr[j]);
                i++;
            }
        }
        Swap(ref arr[i], ref arr[high]);
        return i;
    }

    private static void Swap<T>(ref T a, ref T b)
    {
        T temp = a;
        a = b;
        b = temp;
    }
}