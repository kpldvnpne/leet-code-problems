#include <stdio.h>
#include <iostream>

#include <string>
#include <iostream>
#include <list>

class StringBuilder {
private:
    std::list<std::string> _list;
    int _length = 0;

public:
    void add_front(std::string s) {
        _list.push_front(s);
        _length += s.length();
    }

    void add_back(std::string s) {
        _list.push_back(s);
        _length += s.length();
    }

    int compare(std::string other) {
        auto list_itr = _list.cbegin();
        int other_i = 0;
        int cur_i = 0; 
        for (int i = 0; i < 2; ++i) {
            while (list_itr != _list.cend() && other_i < other.size()) {
                int cmp = (*list_itr)[cur_i] - other[other_i];
                if (cmp == 0) {
                    other_i++;
                    cur_i++;
                    if (cur_i >= (*list_itr).length()) {
                        cur_i -= (*list_itr).length();
                        ++list_itr;
                    }
                } else {
                    return cmp;
                }
            }


            if (other_i >= other.size()) {
                other_i = 0; // Reset to 0 so that the rest can be compared
            } else {
                list_itr = _list.cbegin();
                cur_i = 0;
            }
        }

        if (list_itr == _list.cend() && other_i < other.size()) {
            // Other has some more left
            return -1;
        }

        if (list_itr != _list.cend() && other_i >= other.size()) {
            // List has some more left
            return 1;
        }

        // If everything so far matches
        return 0;
    }

    std::string to_string() {
        std::string s;
        for (auto itr = _list.cbegin(); itr != _list.cend(); ++itr) {
            s.append(*itr);
        }
        return s;
    }

    int length() {
        return _length;
    }
};



// int main(int argc, char **argv) {
//     StringBuilder sb;
//     std::cout << "When empty: aaaa " << sb.compare("aaaa") << std::endl;
//     std::cout << "When empty: bbbb " << sb.compare("bbbb") << std::endl;

//     sb.add_back("aaa");
//     sb.add_front("bbb");

//     std::cout << sb.to_string() << std::endl;

//     // Test comparision
//     std::cout << "For aaaa: " << sb.compare("aaaa") << std::endl;
//     std::cout << "For bbbb: " << sb.compare("bbbb") << std::endl;
// }

int main(int argc, char **argv) {
    int t;
    scanf("%d", &t);


    for (int i = 0; i < t; ++i) {
        int n;
        scanf("%d", &n);

        StringBuilder sb;
        for (int j = 0; j < n; ++j) {
            std::string s;
            std::cin >> s;

            if (sb.compare(s) <= 0) {
                sb.add_back(s);
            } else {
                sb.add_front(s);
            }

            // std::cout << sb.to_string() << std::endl;
        }

        std::cout << sb.to_string() << std::endl;
    }


    return 0;
}