#include <iostream>
#include <string>
#include <sstream>

using namespace std;
struct Link ;
struct Node {
	int data;
	struct Link * next;
	struct Link * prev;
	Node (int d) : data(d), next(0), prev(0) { }
};

struct Link {
	Node * ptr;
	Link () : ptr(0) { }
};

struct List {
	struct Link * head;
	struct Link * tail;
	List() : head(new Link()), tail(new Link()) {  }
	void add_front(int data) {
		Node * n = new Node(data);
		if (head->ptr == 0) {
			// tail must also be 0
			head->ptr = n; tail->ptr = n;
		} else {
			//struct Link * iter_link = head;
			//while (iter_link->next) {
			//	iter_link=iter_link->next;
			//}
			n    -> next               = head;
			if (head->ptr->prev == 0) {
				head -> ptr -> prev = n;
			} else {
				head -> ptr -> prev -> ptr = n;
			}
			head -> ptr                = n;
		}
	}

	string print() {
		stringstream s;
		for (Link * l = head; l != 0; l = l->ptr->next) {
			s << " : " << l->ptr->data ;
		}
		s <<  endl; 
		return s.str(); 
	}
};

void test_add_front()
{
	struct List l ;
	l.add_front(1);
	l.print();
	l.add_front(2);
	l.print();
}

int main()
{
	test_add_front();
}

