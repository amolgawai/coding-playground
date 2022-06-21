#include "const_string.h"

const_string::const_string(char const* s, size_t length) : m_begin(s), m_end(m_begin + length) {
  if (length == 0) erase();
}

size_t const_string::length() const { return m_end - m_begin; }

bool const_string::is_empty() const { return m_end == m_begin; }

void const_string::erase() { m_begin = m_end = ""; }

char const* const_string::data() const { return m_begin; }
