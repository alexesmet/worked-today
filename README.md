# How much did I work?
This program takes in output of my [timesheet-generator](https://github.com/alexesmet/timesheet-generator) to produce a colorful representation of wokring hours for each day, week and month. One of cli tools I actually use on daily basis.

```
D  2023-03-01 Wed  -1.00 (   7.00 /   8 )
D  2023-03-02 Thu        (   8.00 /   8 )
D  2023-03-03 Fri  -4.00 (   4.00 /   8 )
W         Week #9  -5.00 (  19.00 /  24 )
D  2023-03-06 Mon        (   8.00 /   8 )
D  2023-03-07 Tue        (   8.00 /   8 )
D  2023-03-08 Wed  +2.00 (  10.00 /   8 )
D  2023-03-09 Thu        (   8.00 /   8 )
D  2023-03-10 Fri        (   8.00 /   8 )
W        Week #10  +2.00 (  42.00 /  40 )
D  2023-03-13 Mon  +2.00 (  10.00 /   8 )
D  2023-03-14 Tue        (   8.00 /   8 )
D  2023-03-15 Wed        (   8.00 /   8 )
D  2023-03-16 Thu        (   8.00 /   8 )
D  2023-03-17 Fri        (   8.00 /   8 )
W        Week #11  +2.00 (  42.00 /  40 )
D  2023-03-20 Mon  +1.00 (   9.00 /   8 )
D  2023-03-21 Tue        (   8.00 /   8 )
D  2023-03-22 Wed  +2.00 (  10.00 /   8 )
D  2023-03-23 Thu        (   8.00 /   8 )
D  2023-03-24 Fri        (   8.00 /   8 )
W        Week #12  +3.00 (  43.00 /  40 )
D  2023-03-27 Mon  +2.00 (  10.00 /   8 )
D  2023-03-28 Tue  -5.00 (   3.00 /   8 )
W        Week #13  -3.00 (  13.00 /  16 )
M           March  -1.00 ( 159.00 / 160 )

```
(in terminal it's also colorful)


### Technical complications
My challange was to read csv data as a stream, without storing it in memory,
producing the dashboard on-the-fly. Problem is that months don't really consist of weeks, weeks and months exist independently. Great experience working with stream readers!

