fn convolveDriver(matrix: Matrix, source: Matrix, destination: Matrix, iterations: int){
    // Assign the source and destination to swappable variables (by
        // reference, not by value).
        if iterations % 2 > 0{
            map1: Matrix = source;
            map2: Matrix = destination;
        } else{
        // Copy source data into destination so we end up with the
        // destination data in the destination array after an even
        // number of convolutions.
        destination = source;
        map1: Matrix = destination;
        map2: Matrix = source;
    }
        // Loop through the iterations.
        for i in 0..iterations{
        // Run the convolution.
        convolve(matrix, map1, map2);
        
        // Swap the variables.
         map1, map2 = map2, map1
        }

        

        
}
