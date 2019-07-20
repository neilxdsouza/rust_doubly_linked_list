
#include <iostream>
#include <string>
#include <sstream>

using namespace std;

struct Node {
	int data;
	Node (int d) : data(d) { }
};

struct Link {
	struct Node * n;
	struct Link * next;
	struct Link * prev;
	Link (Node * node) : n(node), prev(0), next(0) { }
};

struct List {
	struct Link * head;
	struct Link * tail;
	List(): head(0), tail(0) { }
	void add_front(int data) ;
	string print() ;
};

void List::add_front(int data) {
	Node * n = new Node(data);
	Link * l = new Link(n);
	if (head == 0) {
		head = l;
		tail = l;
	} else {
		l->next = head;
		head->prev = l;
		head = l;
	}
}

string List::print() {
	stringstream s;
	for (Link * l = head; l != 0; l = l->next) {
		s << " : " << l->n->data ;
	}
	s <<  endl; 
	return s.str(); 
}


void test_add_front()
{
	struct List l ;
	l.add_front(1);
	cout << l.print();
	l.add_front(2);
	cout << l.print();
}

int main()
{
	test_add_front();
}

