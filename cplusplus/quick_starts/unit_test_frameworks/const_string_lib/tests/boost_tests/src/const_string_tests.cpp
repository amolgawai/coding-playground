#define BOOST_TEST_MODULE const_string test
#include <boost/test/included/unit_test.hpp>
#include "const_string.h"

BOOST_AUTO_TEST_CASE( constructors_test )
{
     const_string cs0( "" );                                                 // 1 //
     BOOST_CHECK_EQUAL( cs0.length(), (size_t)0 );
     BOOST_CHECK( cs0.is_empty() );
}
// EOF
