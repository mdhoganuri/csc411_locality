# A3 - Locality

## Solution Architecture

Our solution for the locality assignment utilizes the Array2 Library Crate provided by Prof. Daniels, which we extended by creating additional functions for rotation, flipping, and transposition.

First, the program reads and parses the input to determine what operation to perform on the image using either row-major or column-major iteration. From here, we call our functions for rotation, flipping, or transposition accordingly. These functions (in the main) will then call the respective rotation, flip, or transposition (in terms of degrees/horiztonal-vertical) on the Array2 containing our Rgb pixels. Once the proper function from Array2 has played out, the resulting image will be written to the standard output and the runtime of the operations performed will be printed to the standard error.

## Implementation Review

Unlike previous assignments where our confidence in our final solution has been shaky, we are quite happy with our results on locality. Not only did we implement the required functionality - 90 and 180 degree rotation - but we also were able to complete all extra functionalities - 270 degree rotation, horiztonal and vertical flips, and transposition.

We were also happy to be able to run extensive tests on our program, ensuring all functions worked properly with various image inputs.

This assignment took approximately 5 hours to complete.

## Speed Benchmarks (Part C)

| | Row-Major | Col-Major |
| - | - | - |
| 90 Degree | ~17.0538 sec | ~17.6288 sec |
| 180 Degree | ~16.1086 sec | ~19.5982 sec |

The speed benchmarks in the table above vary heavily from our initial projections, however, we have come to realize this is expected as our initial projections were somewhat flawed.

The speeds above make sense, as our Array2 implementation is stored in row-major order, meaning that row-major operations should and do execute faster than column-major operations. That said, the 90 degree rotation speeds are quite similar as they are both about even in how they hit and miss in cache, it's more a matter of load vs store. The 180 degree rotation is a bit clearer as our 180 degree, row-major rotation will obviously hit much more frequently given our row-major storing of values in Array2. Our 180 degree, column-major rotation, on the other hand, will be a cache miss nearly every iteration as this is not how our Array2 is stored.

## Contributors

Matt Hogan  
Mike Cavallaro  
Prof. Daniels (Array2 Library Crate)
