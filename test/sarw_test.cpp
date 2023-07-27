#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "sarw.hpp"

#include "doctest/doctest.h"
#include "region.hpp"

TEST_CASE("testing single walk sarw") {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vector3D<double>(0, 0, 0), 10);

  molpack::SARW sarw(std::move(region), 1);

  auto result = sarw.walk(100);

  CHECK(result.size() == 100);

  double min_distance = 100;
  for (std::size_t i = 0; i < result.size(); i++) {
    for (std::size_t j = i + 1; j < result.size(); j++) {
      double distance = (result[i] - result[j]).norm2();
      if (distance < min_distance) {
        min_distance = distance;
      }
    }
  }

  CHECK(min_distance == doctest::Approx(1));
}