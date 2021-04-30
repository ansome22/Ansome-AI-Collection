// The strength fn has this format.
fn strengthfn(city: City, location: Location) -> f64
// This structure is used to keep track of the information we need for
// each location.
6 class LocationRecord:
7 location: Location
8 nearestCity: City
9 strength: f64
10
11 fn mapfloodDijkstra(map: Map,
12 cities: City[],
13 strengthThreshold: f64,
14 strengthfn: fn)
15 -> LocationRecord[]:
16
// Initialize the open and closed lists.
18 open = new PathfindingList()
19 closed = new PathfindingList()
20
// Initialize the record for the start nodes.
22 for city in cities:
23 startRecord = new LocationRecord()
24 startRecord.location = city.getLocation()
25 startRecord.city = city
26 startRecord.strength = city.getStrength()
27 open += startRecord
28
// Iterate through processing each node.
30 while open:
// Find the largest element in the open list.
32 current = open.largestElement()
33
// Get its neighboring locations.
35 locations = map.getNeighbors(current.location)
36
// Loop through each location in turn.
38 for location in locations:
// Get the strength for the end node.
40 strength = strengthfn(current.city, location)
41
// Skip if the strength is too low.
43 if strength < strengthThreshold:6.2 Tactical Analyses 531
44 continue
45
// .. or if closed and we’ve found a worse route.
47 else if closed.contains(location):
// Find the record in the closed list.
49 neighborRecord = closed.find(location)
50 if neighborRecord.city != current.city and
51 neighborRecord.strength < strength:
52 continue
53
// .. or if it is open and we’ve found a worse
// route.
56 else if open.contains(location):
// Find the record in the open list.
58 neighborRecord = open.find(location)
59 if neighborRecord.strength < strength:
60 continue
61
// Otherwise we know we’ve got an unvisited
// node, so make a record for it.
64 else:
65 neighborRecord = new NodeRecord()
66 neighborRecord.location = location
67
// We’re here if we need to update the node
// Update the cost and connection.
70 neighborRecord.city = current.city
71 neighborRecord.strength = strength
72
// And add it to the open list.
74 if not open.contains(location):
75 open += neighborRecord
76
// We’ve finished looking at the neighbors for the current
// node, so add it to the closed list and remove it from the
// open list.
80 open -= current
81 closed += current
82
// The closed list now contains all the locations that belong to
// any city, along with the city they belong to.
85 return closed