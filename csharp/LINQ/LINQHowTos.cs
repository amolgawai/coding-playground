
// Remove null items 

IEnumerable<object> listObjects = new Collection<object> { 1, null, 5, 6, 10, 12, null };
calculationRequests.RemoveAll( item => item == null );


// Compare two lists to find items not in each other

IEnumerable<int> list1 = new List<int> { 1, 2, 5, 6, 10, 12, 33 };
IEnumerable<int> list2 = new List<int> { 1, 2, 5, 6, 7, 8, 9 };

IEnumerable<int> list1Exceptlist2 =  list1.Except( list2 );

IEnumerable<int> list2Exceptlist2 =  list2.Except( list1 );


// Compare two lists to find items common each other

IEnumerable<int> list1 = new List<int> { 1, 2, 5, 6, 10, 12, 33 };
IEnumerable<int> list2 = new List<int> { 1, 2, 5, 6, 7, 8, 9 };
IEnumerable<int> commonItemList = list1.Intersect( list2);

// Compare two lists of collections with a comaparator - http://stackoverflow.com/questions/1076144/comparing-two-collections-of-objects


// Find item with specific condition [X]Complete this

IEnumerable<int> list1 = new List<int> { 1, 2, 5, 6, 10, 12, 33 };
var perticularItems = list1.Where( item => item == 10)
     
