package main

import (
	"fmt"
	"sort"
	"strings"
)


func PlaySort(){
	// Float64s(): Sorts a slice of float64
	list_of_floats := []float64{5.2, -1.3, 0.7, -3.8, 2.6} // unsorted
	sort.Float64s(list_of_floats)

	// Ints(): sort a list of ints
	list_of_ints := []int{5, 2, 6, 3, 1, 4} // unsorted
	sort.Ints(list_of_ints)

	// Strings(): sort a list of strings
	list_of_strings := []string{"a", "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z"}
	sort.Strings(list_of_strings)


	// Slice(): sort with the provided Less function
	// SliceStable() keeps equal elements in their original order
	sort.Slice(list_of_floats, func(i, j int) bool{ return list_of_floats[i] < list_of_floats[j] })
	//sorted := sort.SliceIsSorted(...)


	// Sort() sorts data as determined by the Less method
	// Stable() sorts data, keeping the original order of equal elements
	people := []PersonWithAge{
		{"Bob", 31},
		{"John", 42},
		{"Michael", 17},
		{"Jenny", 26},
	}
	sort.Sort(ByAge(people)) // cast to a sortable interface

	// Check is sorted
	sort.Float64sAreSorted(list_of_floats)
	sort.IntsAreSorted(list_of_ints)
	sort.StringsAreSorted(list_of_strings)
	sort.IsSorted(ByAge([]PersonWithAge{}))  // check sort.Interface


	// Find(): binary search to find the smallest [i] at which cmp(i)<=0.	
	// Requires: cmp(i) starts with >0, hits ==0, then remains <0 until the end of the list
	target := "k"
	i, found := sort.Find(len(list_of_strings), func(i int) int { return strings.Compare(target, list_of_strings[i]) })
	fmt.Printf("Found [%d]%q (%t)\n", i, list_of_strings[i], found)

	// Search(): binary search to find the smallest [i] at which f(i)==true
	// Requires: f(i) starts as false, then switches to true, and remains so until the end of the list
	i = sort.Search(len(list_of_strings), func(i int) bool { return list_of_strings[i] == "k" })
	if i < len(list_of_strings) {
		fmt.Printf("Found [%d]%q\n", i, list_of_strings[i])
	} else {
		fmt.Printf("Not found\n")
	}

	// Search*(): return the index to insert <value> if it is not present.
	i = sort.SearchInts(list_of_ints, 4)
	i = sort.SearchFloat64s(list_of_floats, 0.0)
	i = sort.SearchStrings(list_of_strings, "k")
	fmt.Printf("Found: [%d]%q\n", i, list_of_strings[i])
}


func PlayCustomSort() {
	people := []PersonWithAge{
		{"Bob", 31},
		{"John", 42},
		{"Michael", 17},
		{"Jenny", 26},
	}

	// Sort people by Age: use the implementation of the sort.Interface
	sort.Sort(ByAge(people))
	
	// Sort people by Age: use a custom Less function
	sort.Slice(people, func(i, j int) bool {
		return people[i].Age > people[j].Age
	})

	// (!) Programmable sort keys
	sorters := map[string]lessFunc{
		"name": func(p1, p2 *PersonWithAge) bool {
			return p1.Name < p2.Name
		},
		"age": func(p1, p2 *PersonWithAge) bool {
			return p1.Age < p2.Age 
		},
	}
	By(sorters["age"]).Sort(people)

	// (!) Programmable multi-sort keys
	OrderedBy(sorters["name"], sorters["age"]).Sort(people)
	

	// (!) Sort wrapper
	// Struct wraps the list.
	sort.Sort(ByName{people})
	

	// Done
	fmt.Println(people)
}



type PersonWithAge struct {
	Name string
	Age  int
}

// === Sorter by Age
// Implement sort.Interface 
type ByAge []PersonWithAge
func (a ByAge) Len() int           { return len(a) }
func (a ByAge) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByAge) Less(i, j int) bool { return a[i].Age < a[j].Age }

//===  Programmable Less() function
type By func(p1, p2 *PersonWithAge) bool
func (by By) Sort(people []PersonWithAge){
	sort.Sort(&peopleSorter{people: people, by: by})
}
// Implement sort.Interface
type peopleSorter struct {
	people []PersonWithAge
	by By
}
func (ps *peopleSorter) Len() int { return len(ps.people) }
func (ps *peopleSorter) Swap(i, j int) { ps.people[i], ps.people[j] = ps.people[j], ps.people[i] }
func (ps *peopleSorter) Less(i, j int) bool { return ps.by(&ps.people[i], &ps.people[j]) }


// === Programmable Less() func with multiple sort fields
type lessFunc func(p1, p2 *PersonWithAge) bool
func OrderedBy(less ...lessFunc) *multiSorter {
	return &multiSorter{less: less}
}

type multiSorter struct {
	less []lessFunc
	people []PersonWithAge
}
func (ms *multiSorter) Sort(people []PersonWithAge){
	ms.people = people
	sort.Sort(ms)
}

// Implement sort.Interface
func (ms *multiSorter) Len() int { return len(ms.people) }
func (ms *multiSorter) Swap(i, j int) { ms.people[i], ms.people[j] = ms.people[j], ms.people[i] }
func (ms *multiSorter) Less(i, j int) bool {
	p1, p2 := &ms.people[i], &ms.people[j]

	// Run every function in turn. If it definitely says "less", then return.
	// If it says ">=" -- proceed to the next function.
	// When all comparisons have been tried, return whatever the last comparison returns
	for _, less := range ms.less {
		switch {
		case less(p1, p2): return true
		case less(p2, p1): return false
		}
	}
	
	// If all comparisons have been tried, then return whatever the last comparison returns.
	// It should be a false
	return false	
}


// === Alternative: Sort wrapper
// Common methods to all people
type People []PersonWithAge
func (s People) Len() int      { return len(s) }
func (s People) Swap(i, j int) { s[i], s[j] = s[j], s[i] }

// Special sorter by name
type ByName struct{People}
func (s ByName) Less(i, j int) bool { return s.People[i].Name < s.People[j].Name }
