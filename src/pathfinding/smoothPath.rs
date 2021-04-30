
fn smoothPath(inputPath: Vector[]) -> Vector[]{
        // If the path is only two nodes long, then we can’t smooth it, so
        // return.
        if len(inputPath) == 2{
            return inputPath
        }

        // Compile an output path.
        outputPath = [inputPath[0]]

        // Keep track of where we are in the input path. We start at 2,
        // because we assume two adjacent nodes will pass the ray cast.
        inputIndex: int = 2

        // Loop until we find the last item in the input.
        while inputIndex < len(inputPath) - 1{
            // Do the ray cast.
            fromPt = outputPath[len(outputPath) - 1];
            toPt = inputPath[inputIndex];
            if !rayClear(fromPt, toPt){
                // The ray cast failed, add the last node that passed to
                // the output list.
                outputPath += inputPath[inputIndex - 1];

                // Consider the next node.
                inputIndex ++;
            }

            // We’ve reached the end of the input path, add the end node to the
            // output and return it.
            outputPath += inputPath[len(inputPath) - 1];
    }

    return outputPath
}

