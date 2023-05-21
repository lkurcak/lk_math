# lk_math
Tools for solving commonly occuring mathematical problems.

## Rationale.
After having worked on various problems, I've found some share an underlying core difficulty. This crate aims to provide general solutions to these core problems.

### Example.
Line rasterisation. We want to calculate which pixels of an image should be drawn to. We can start at one of the points and iteratively take steps towards the other. At each iteration, we take the step that diverges the least from the ideal line, until we reach the other end.

Another scenario. Several small political parties are forming a united one. They need to create an ordered list of candidates. Each party wants to have as many of their own candidates taking up higher spots. We know that the biggest party should get the first spot, but who should get the second, third, fourth, and so forth, such that it stays as fair as possible?

We can reduce the problem to drawing a rasterized line to an N-dimensional "image" where N is the number of parties. We draw a line from the origin, towards the point with coordinates representing each of the parties' size, until we have drawn as many "pixels" as is the total number of candidates needed.

Both problems can be solved using a general algorithm. The core difficulty is drawing a rasterized line in N-dimensional space. This library aims to provide these abstract general solutions. Note that making the connection can be tricky. Usually, I recognize it halfway while solving new problems from scratch.

## Disclaimer.
Right now, this library serves for my personal use. I mostly wanted to go through the process of publishing a crate and make the features easily available in other projects when I need them. The library is not documented and the API might not be user-friendly.

I would be very happy for anyone to try it though, just have the understanding not much thought went into making it intuitive as of now. Feel free to ping me, open issues or contribute. It would be super cool if someone found it useful!
