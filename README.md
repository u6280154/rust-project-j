# Funpar Project

Work done by :
Suppachoke Areechitsakul 6281553 | jay-aree/JayAree999 (2 accounts)
Sumet Saelow 6280154 | Unknobn

How the project works:

We implemented various sorting algorithms and race it between each other and see who runs the fastest.
List of sortings algorithms used : Bubble Sort, Radix Sort, Sample Sort and , Quick Sort.
These sorts will be implemented in parallel and see how these improve the performance. 

Our findings:

| Sorting Algorithm               | 10000 N | 100000 N | 1000000 N |
|---------------------------------|---------|----------|-----------|
| seq_quicksort                   | 11 ms   | 144 ms   | 1780 ms   |
| par_quicksort                   | 14 ms   | 138 ms   | 1697 ms   |
| par_samplesort_with_handmade_qs | 23 ms   | 166 ms   | 1670 ms   |
| par_sample_sort                 | 22 ms   | 157 ms   | 1605 ms   |
| sample_sort_with_handmade_qs    | 12 ms   | 129 ms   | 8347 ms   |
| sample_sort                     | 10 ms   | 110 ms   | 7055 ms   |
| par_radix_sort                  | 31 ms   | 149 ms   | 1709 ms   |
| radix_sort_from_crate           | 16 ms   | 139 ms   | 1513 ms   |
| bubble_sort                     | 2861 ms | x        | x         |
| par_bubble_sort                 | 2035 ms | x        | x         |
|                                 |         |          |           |
|                                 |         |          |           |

*bubble sort and par bubble sort can't survive 100 000 or more array unit
(I believe in you now when you say unorthodox)




