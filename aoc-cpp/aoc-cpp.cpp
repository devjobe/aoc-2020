#include <iostream>
#include <fstream>
#include <sstream>
#include <iterator>
#include <algorithm>
#include <unordered_set>
#include <set>
#include <unordered_map>
#include <string>
#include <numeric>
#include <functional>
#include <charconv>

using namespace std;

std::string slurp(ifstream &stream) {
  stringbuf buf;
  stream >> &buf;
  return buf.str();
}

std::string readall(std::string_view path) { 
  ifstream stream(string{path}); 
  return slurp(stream);
}

// Split on "\r\n\r\n", "\n\n", "\n\r\n", or "\r\n\n"
std::pair<std::string_view, std::string_view> split_paragraph(string_view s) {
  for (size_t off = 0; (off = s.find_first_of("\r\n", off)) != s.npos;
       off += 1) {
    if (s[off] == '\n') {
      if (off + 1 == s.size() || s[off + 1] == '\n') {
        return {s.substr(0, off), s.substr(off + 2)};
      } else if (s.substr(off + 1, 2) == "\r\n"sv) {
        return {s.substr(0, off), s.substr(off + 3)};
      }
    } else if (s[off] == '\r') {
      auto nl = s.substr(off + 1, 3);
      if (nl == ""sv || nl == "\n\r\n"sv || nl == "\n"sv) {
        return {s.substr(0, off), s.substr(off + 1 + nl.size())};
      }
      if (nl.size() && nl[0] == '\n')
        off += 1;
    }
  }

  return {s, {}};
}

std::pair<std::string_view, std::string_view> split_line(string_view s) {
  size_t off = s.find_first_of("\r\n");
  if (off == s.npos) {
    return {s, {}};
  } else if (s[off] == '\r' && off + 1 < s.size() && s[off + 1] == '\n') {
    return {s.substr(0, off), s.substr(off + 2)};
  } else {
    return {s.substr(0, off), s.substr(off + 1)};
  }
}

std::pair<string_view, string_view> split_once(string_view s, size_t ndx,
                                               size_t remove_count = 1) {
  if (ndx == s.npos) {
    return {s, {}};
  } else {
    return {s.substr(0, ndx), s.substr(ndx + remove_count)};
  }
  
}


template <typename T> struct range_t {
  T begin_;
  T end_;

  range_t() = default;
  range_t(T begin, T end) : begin_(begin), end_(end) {}

  auto begin() const { return begin_; }
  auto end() const { return end_; }

  range_t skip(size_t n) const {
    auto it = begin_;
    while (n > 0) {
      n--;
      ++it;
    }
    return {it, end_};
  }
};


template <typename SplitPattern>
struct StringSplitIterator {
  StringSplitIterator() = default;
  StringSplitIterator(string_view range, SplitPattern pattern = {})
      : pattern_(pattern), range_(range), item_(pattern_(range_)) {}

  StringSplitIterator &operator++(void) {
    item_ = pattern_(range_);
    return (*this);
  }

  StringSplitIterator operator++(int) {
    StringSplitIterator temp(*this);
    ++*this;
    return (temp);
  }

  bool operator==(const StringSplitIterator &rhs) {
    return item_.data() == rhs.item_.data();
  }

  bool operator!=(const StringSplitIterator &rhs) {
    return item_.data() != rhs.item_.data();
  }

  string_view operator*() const { return item_; }

private:
  SplitPattern pattern_ = {};
  string_view range_ = {};
  string_view item_ = {};
};

struct CharSplitPattern {
  char ch = {};

  string_view operator()(string_view &view) const {
    auto pos = view.find_first_of(ch);
    if (pos == view.npos) {
      auto res = view;
      view = {};
      return res;
    } else {
      auto res = view.substr(0, pos);
      view.remove_prefix(pos + 1);
      return res;
    }
  }
};

range_t<StringSplitIterator<CharSplitPattern>> split(string_view view,
                                                            char ch) {
  return {{view, {ch}}, {}};
}


struct WordSplitPattern {
  
  string_view operator()(string_view &view) const {
    auto beg = view.find_first_not_of(" \t\r\n");
    if (beg == view.npos) {
      view = {};
      return {};
    }

    auto pos = view.find_first_of(" \t\r\n", beg);
    if (pos == view.npos) {
      auto res = view.substr(beg);
      view = {};
      return res;
    } else {
      auto res = view.substr(beg, pos - beg);
      view.remove_prefix(pos);
      return res;
    }
  }
};

range_t<StringSplitIterator<WordSplitPattern>> words(string_view view){
  return {{view}, {}};
}


struct LineSplitPattern {
  string_view operator()(string_view &view) const {
    auto [res, rem] = split_line(view);
    view = rem;
    return res;
  }
};

range_t<StringSplitIterator<LineSplitPattern>>
lines(string_view view) {
  return {{view}, {}};
}

struct ParagraphSplitPattern {
  string_view operator()(string_view &view) const {
    auto [res, rem] = split_paragraph(view);
    view = rem;
    return res;
  }
};

range_t<StringSplitIterator<ParagraphSplitPattern>>
paragraphs(string_view view) {
  return {{view}, {}};
}

bool in_range(std::pair<int, int> min_max, string_view s) {

  int n;
  auto end = s.data() + s.size();
  auto [ptr, ec] = from_chars(s.data(), s.data() + s.size(), n);

  bool res = (ec == std::errc()) && ptr == end && n >= min_max.first &&
             n <= min_max.second;
  return res;
}

bool ends_with(string_view s, string_view pattern) {
  return s.size() >= pattern.size() &&
         s.compare(s.size() - pattern.size(), pattern.size(), pattern) == 0;
}



void day1_part1() {
  ifstream stream("../input/day1.txt");
  unordered_set<int> set(istream_iterator<int>{stream}, {});
  auto it = find_if(set.begin(), set.end(),
                    [&](auto a) { return set.find(2020 - a) != set.end(); });
  cout << (*it * (2020 - *it)) << endl;
}

void day1_part2() {
  ifstream stream("../input/day1.txt");
  unordered_set<int> set(istream_iterator<int>{stream}, {});
  
  auto it = set.begin();
  for (auto a = *it++; it != set.end();a = *it++) {
    auto res = find_if(it, set.end(),
                [&](auto b) { return set.find(2020 - a - b) != set.end(); });
    if (res != set.end()) {
      auto b = *res;
      cout << a * b * (2020 - a - b) << endl;
      break;
    }
  }
}

void day2_part1() {
  ifstream stream("../input/day2.txt");
  std::string line;

  int valid = 0;
  while (!stream.eof()) {
    int min;
    int max;
    char ch, tmp;
    stream >> min >> tmp >> max >> ws >> ch >> tmp >> ws;
    std::getline(stream, line);

    auto n = count(line.begin(), line.end(), ch);
    valid += (n >= min && n <= max);
  }
  
  cout << valid << endl;
}


void day2_part2() {
  ifstream stream("../input/day2.txt");  
  string line;
  int valid = 0;
  while (!stream.eof()) {
    size_t a;
    size_t b;
    char ch, tmp;
    stream >> a >> tmp >> b >> ws >> ch >> tmp >> ws;
    getline(stream, line);

    valid += (a <= line.size() && line[a - 1] == ch) !=
             (b <= line.size() && line[b - 1] == ch);
  }

  cout << valid << endl;
}

void day3_part1() { 
  ifstream stream("../input/day3.txt");
  string line;
  int trees = 0;
  for (size_t x = 0; !stream.eof();x += 3) {
    getline(stream, line);
    trees += line[x % line.size()] == '#';
  }

  cout << trees << endl;
}

void day3_part2() {

  struct {
    size_t dx;
    size_t dy;
    size_t trees;
  } walks[] = {
    {1, 1, 0}, {3, 1, 0}, {5, 1, 0}, {7, 1, 0}, {1, 2, 0},
  };

  ifstream stream("../input/day3.txt");
  string line;
  for (size_t y = 0; !stream.eof(); ++y) {
    getline(stream, line);
    for (auto &w : walks) {
      w.trees +=
          (y % w.dy) == 0 && line[(w.dx * y / w.dy) % line.size()] == '#';
    }
  }

  uint64_t prod = 1;
  for (auto const &w : walks) {
    prod *= w.trees;
  }

  cout << prod << endl;
}

void day4_part1() {
  unordered_map<string_view, int> keys = {
      {"cid", 1 << 0}, {"ecl", 1 << 1}, {"pid", 1 << 2}, {"eyr", 1 << 3},
      {"hcl", 1 << 4}, {"byr", 1 << 5}, {"iyr", 1 << 6}, {"hgt", 1 << 7},
  };

  string data = readall("../input/day4.txt");
  int valid = 0;
  for (auto p : paragraphs(data)) {
    int set = 0;
    for (auto kw : words(p)) {
      auto [key, value] = split_once(kw, kw.find_first_of(':'));
      auto iter = keys.find(key);
      if (iter == keys.end() || (set & iter->second))
        set = -1;
      else
        set |= iter->second;
    }

    if (set != -1 && (set & 254) == 254) {
      valid++;
    }
  }
  cout << valid << endl;

}



void day4_part2() {
  
  unordered_set<string_view> ecl_set = {"amb", "blu", "brn", "gry",
                                        "grn", "hzl", "oth"};
  unordered_map<string_view, std::pair<int, std::function<bool(string_view)>>>
      keys = {
          {"cid", {1 << 0, [](auto sv) { return true; }}},
          {"ecl",
           {1 << 1,
            [&](auto sv) { return ecl_set.find(sv) != ecl_set.end(); }}},
          {"pid",
           {1 << 2,
            [](auto sv) {
              bool res = sv.size() == 9 &&
                         count_if(sv.begin(), sv.end(), isdigit) == 9;
              return res;
            }}},
          {"eyr",
           {1 << 3,
            [](auto sv) {
              return in_range({2020, 2030}, sv);
            }}},
          {"hcl",
           {1 << 4,
            [](auto sv) {
              return sv.size() == 7 && sv[0] == '#' &&
                     count_if(sv.begin(), sv.end(), [](auto x) {
                       return isdigit(x) || (x >= 'a' && x <= 'z');
                     }) == 6;
            }}},
          {"byr",
           {1 << 5,
            [](auto sv) {
              return in_range({1920, 2002}, sv);
            }}},
          {"iyr",
           {1 << 6,
            [](auto sv) {
              return in_range({2010, 2020}, sv);
            }}},
          {"hgt",
           {1 << 7,
            [](auto sv) {
              return (ends_with(sv, "cm") &&
                      in_range({150, 193}, sv.substr(0, sv.size() - 2))) ||
                     (ends_with(sv, "in") &&
                      in_range({59, 76}, sv.substr(0, sv.size() - 2)));
            }}},
      };

  
  string data = readall("../input/day4.txt");
  int valid = 0;
  for (auto p : paragraphs(data)) {
    int set = 0;
    for (auto kw : words(p)) {
      auto [key, value] = split_once(kw, kw.find_first_of(':'));
      auto iter = keys.find(key);
      if (iter == keys.end() || (set & iter->second.first) ||
          !iter->second.second(value)) {
        set = -1;
      } else {
        set |= iter->second.first;
      }
    }

    if (set != -1 && (set & 254) == 254) {
      valid++;
    }
  }
  cout << valid << endl;
}


void day5_part1() {
  string data = readall("../input/day5.txt");
  int max_id = 0;
  for (auto line : lines(data)) {
    max_id = max(max_id, std::accumulate(line.begin(), line.end(), 0,
                                         [](auto acc, auto c) {
                                           return (acc << 1) | ((c & 0xf) == 2);
                                         }));
  }
  cout << max_id << endl;
}


void day5_part2() {
  string data = readall("../input/day5.txt");
  std::vector<int> seats;
  for (auto line : lines(data)) {
    int seat_id =
        std::accumulate(line.begin(), line.end(), 0, [](auto acc, auto c) {
          return (acc << 1) | ((c & 0xf) == 2);
        });

    seats.insert(upper_bound(seats.begin(), seats.end(), seat_id), seat_id);
  }
  auto it = adjacent_find(seats.begin(), seats.end(),
                          [](auto a, auto b) { return b - a == 2; });
  cout << (*it + 1) << endl;
}

void day6_part1() {
  string input = readall("../input/day6.txt");

  int answer = 0;
  for (auto paragraph : paragraphs(input)) {
    unordered_set<char> set;
    set.reserve(paragraph.size());
    for (auto it : paragraph) {
      if (!isspace(it)) {
        set.insert(it);
      }
    }
    answer += set.size();
  }

  cout << answer << endl;
}

void day6_part2() {
  string input = readall("../input/day6.txt");

  int answer = 0;
  for (auto paragraph : paragraphs(input)) {
    auto list = lines(paragraph);
    string intersection(*list.begin());
    std::sort(intersection.begin(), intersection.end());
    for (auto line : list.skip(1)) {

      string next(line);
      std::sort(next.begin(), next.end());

      string result;
      set_intersection(intersection.begin(), intersection.end(), next.begin(),
                       next.end(), back_inserter(result));

      intersection = result;
    }
    answer += intersection.size();
  }
  cout << answer << endl;
}



void day7_part1() {
  string input = readall("../input/day7.txt");

  int answer = 0;
  for (auto paragraph : lines(input)) {
  }
  cout << answer << endl;
}


int main() {
  day7_part1();
  
  return 0;
}