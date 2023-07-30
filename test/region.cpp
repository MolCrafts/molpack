#include "region.hpp"

#include <memory>

#include "doctest/doctest.h"

TEST_CASE("testing with SphericalRegion") {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vec3<double>(0, 0, 0), 10);

  CHECK(region->isInside(0, 0, 0));
  CHECK(region->isInside(1, 1, 1));
  CHECK(!region->isInside(6, 6, 6));
  CHECK(!region->isInside(100, 100, 100));

  CHECK(region->size().getX() == doctest::Approx(20));
  CHECK(region->size().getY() == doctest::Approx(20));
  CHECK(region->size().getZ() == doctest::Approx(20));

  CHECK(region->origin().getX() == doctest::Approx(-10));
  CHECK(region->origin().getY() == doctest::Approx(-10));
  CHECK(region->origin().getZ() == doctest::Approx(-10));
}