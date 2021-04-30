// Perform a convolution of a matrix that is the outer product of the
// given vectors, on the given source.
fn separableConvolve(hvector: Vector,vvector: Vector,source: Matrix, temp: Matrix, destination: Matrix){
    // Find the size of the vectors.
    vectorLength: int = hvector.length();
    size: int = (vectorLength - 1) / 2;

    // Find the dimensions of the source.
    height: int = source.length();
    width: int = source[0].length();

    // Go through each destination node, missing out a border equal to
    // the size of the vector.
    for i in size..(width - size){
        for j in size..(height - size){
            // Start with zero in the temp array.
            temp[i][j] = 0;

            // Go through each entry in the vector.
            for k in 0..vectorLength{
                // Add the component.
                temp[i][j] += source[i][j + k - size] * vvector[k]
                // Go through each destination node again.
                for i in size..(width - size){
                    for j in size..(height - size){
                        // Start with zero in the destination.
                        destination[i][j] = 0;
                        
                        // Go through each entry in the vector.
                        for k in 0..vectorLength{
                        // Add the component (taking data from temp rather than
                        // the source).
                        destination[i][j] += temp[i + k - size][j] * hvector[k];
                        }
                    }
                }
            }
        }
    }
}