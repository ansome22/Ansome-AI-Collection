mod TranspositionTable{
// Holds a single table entry.
 pub struct Entry{
// The world model for the entry, all entries are initially
// empty.428 Chapter 5 Decision Making
 worldModel = null,
}


// The depth that the world model was found at. This is
// initially infinity, because the replacement strategy we use
// in the add method can then treat entries the same way
// whether they are empty or not.
 depth = infinity

// A fixed size array of entries.
size: int
entries: Entry[size]

fn has(worldModel: WorldModel) -> bool{
// Get the hash value.
hashValue: int = hash(worldModel)

// Find the entry.
 entry: Entry = entries[hashValue % size]

// Check if is the right one.
 return entry.worldModel == worldModel
}
fn add(worldModel: WorldModel, depth: int){
// Get the hash value.
 hashValue: int = hash(worldModel);

// Find the entry.
 entry: Entry = entries[hashValue % size];

// Check if it is the right world model.
 if entry.worldModel == worldModel{
// If we have a lower depth, use the new one.
 if depth < entry.depth{
    entry.depth = depth
 }
// Otherwise we have a clash (or an empty slot).
} else{
// Replace the slot if our new depth is lower.
if depth < entry.depth{
    entry.worldModel = worldModel
     entry.depth = depth
}
}


}

}