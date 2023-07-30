#include "vec3.hpp"

#include <cmath>

#include "doctest/doctest.h"

TEST_CASE("testing the vec3 algorithm with int") {
  molpack::Vec3<int> v1(1, 2, 3);
  molpack::Vec3<int> v2(4, 5, 6);

  CHECK(v1.getX() == 1);
  CHECK(v1.getY() == 2);
  CHECK(v1.getZ() == 3);

  CHECK(v2.getX() == 4);
  CHECK(v2.getY() == 5);
  CHECK(v2.getZ() == 6);

  auto v3 = v1 + v2;
  CHECK(v3.getX() == 5);
  CHECK(v3.getY() == 7);
  CHECK(v3.getZ() == 9);

  auto v4 = v1 - v2;
  CHECK(v4.getX() == -3);
  CHECK(v4.getY() == -3);
  CHECK(v4.getZ() == -3);

  auto v5 = v1 * 2;
  CHECK(v5.getX() == 2);
  CHECK(v5.getY() == 4);
  CHECK(v5.getZ() == 6);

  auto v6 = v1.norm2();
  CHECK(v6 == doctest::Approx(std::sqrt(14)));
}

TEST_CASE("testing the vec3 algorithm with double") {
  molpack::Vec3<double> v1(1.1, 2.2, 3.3);
  molpack::Vec3<double> v2(4.4, 5.5, 6.6);

  CHECK(v1.getX() == doctest::Approx(1.1));
  CHECK(v1.getY() == doctest::Approx(2.2));
  CHECK(v1.getZ() == doctest::Approx(3.3));

  CHECK(v2.getX() == doctest::Approx(4.4));
  CHECK(v2.getY() == doctest::Approx(5.5));
  CHECK(v2.getZ() == doctest::Approx(6.6));

  auto v3 = v1 + v2;
  CHECK(v3.getX() == doctest::Approx(5.5));
  CHECK(v3.getY() == doctest::Approx(7.7));
  CHECK(v3.getZ() == doctest::Approx(9.9));

  auto v4 = v1 - v2;
  CHECK(v4.getX() == doctest::Approx(-3.3));
  CHECK(v4.getY() == doctest::Approx(-3.3));
  CHECK(v4.getZ() == doctest::Approx(-3.3));

  auto v5 = v1 * 2.5;
  CHECK(v5.getX() == doctest::Approx(2.75));
  CHECK(v5.getY() == doctest::Approx(5.5));
  CHECK(v5.getZ() == doctest::Approx(8.25));

  auto v6 = v1.norm2();
  CHECK(v6 == doctest::Approx(std::sqrt(16.94)));
}

TEST_CASE("testing the vec3 algorithm with double and int mixed") {
  molpack::Vec3<double> v1(1.1, 2.2, 3.3);
  molpack::Vec3<int> v2(4, 5, 6);

  CHECK(v1.getX() == doctest::Approx(1.1));
  CHECK(v1.getY() == doctest::Approx(2.2));
  CHECK(v1.getZ() == doctest::Approx(3.3));

  CHECK(v2.getX() == 4);
  CHECK(v2.getY() == 5);
  CHECK(v2.getZ() == 6);

  auto v3 = v1 + v2;
  CHECK(v3.getX() == doctest::Approx(5.1));
  CHECK(v3.getY() == doctest::Approx(7.2));
  CHECK(v3.getZ() == doctest::Approx(9.3));

  auto v4 = v1 - v2;
  CHECK(v4.getX() == doctest::Approx(-2.9));
  CHECK(v4.getY() == doctest::Approx(-2.8));
  CHECK(v4.getZ() == doctest::Approx(-2.7));

  auto v5 = v2 * 2.5;
  CHECK(v5.getX() == doctest::Approx(10.0));
  CHECK(v5.getY() == doctest::Approx(12.5));
  CHECK(v5.getZ() == doctest::Approx(15.0));
}