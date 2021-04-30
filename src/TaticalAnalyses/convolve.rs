// Performs a convolution of the matrix on the source.
fn convolve(matrix: Matrix, source: Matrix, destination: Matrix){
    // Find the size of the matrix.
    matrixLength: int = matrix.length();
    size: int = (matrixLength - 1) / 2;

    // Find the dimensions of the source.
    height: int = source.length();
    width: int = source[0].length();

    // Go through each destination node, missing out a border equal to
    // the size of the matrix.536 Chapter 6 Tactical and Strategic AI
    for i in size..(width - size){
        for j in size..(height - size){
            // Start with zero in the destination.
            destination[i][j] = 0

            // Go through each entry in the matrix.
            for k in 0..matrixLength{
                for m in 0..matrixLength{
                    // Add the component.
                    destination[i][j] += source[i + k - size][j + m - size] * matrix[k][m];
            }
        }
    }
}
}
