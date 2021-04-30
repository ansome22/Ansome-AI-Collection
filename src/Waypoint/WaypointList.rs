
pub trait WaypointList {
// Initializes the iterator to move in order of increasing value.
fn sort(){}
// Initializes the iterator to move in order of decreasing value.
fn sortReversed(){}
// Return a new waypoint list containing those waypoints that are
// near to the given one.
fn getNearby(waypoint: Waypoint) -> WaypointList{}
// Return the next waypoint in the iteration. Iterations are
// initialized by a call to one of the sort functions. Note that
// this function must work in such a way that remove() can be
// called between calls to next() without causing problems.
fn next() -> Waypoint{}
// Remove the given waypoint from the list.
fn remove(waypoint: Waypoint){}
}