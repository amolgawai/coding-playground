#ifndef CONST_STRING_H
#define CONST_STRING_H

#include <string>

class const_string {
  public:
  // Constructors
  const_string() : m_begin(""), m_end(m_begin) {}
  const_string(std::string const& s) : m_begin(s.c_str()), m_end(m_begin + s.length()) {}
  const_string(char const* s)
      : m_begin(s ? s : ""), m_end(s ? m_begin + std::strlen(s) : m_begin) {}
  const_string(char const* first, char const* last) : m_begin(first), m_end(last) {}
  const_string(char const* s, size_t length);
  // const_string( char const* begin, char const* end );

  // Access methods
  char const* data() const;
  size_t length() const;
  bool is_empty() const;
  void erase();

  private:
  // Data members
  char const* m_begin;
  char const* m_end;
};

#endif
