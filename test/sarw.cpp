#include "sarw.hpp"

#include "doctest/doctest.h"
#include "region.hpp"
#include "vec3.hpp"

TEST_CASE("testing single walk sarw") {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vec3<double>(0, 0, 0), 10);

  molpack::SARW sarw(std::move(region), 1);

  auto result = sarw.walk(100);

  CHECK(result.size() == 100);

  double min_distance = 100;
  for (std::size_t i = 0; i < result.size(); i++) {
    for (std::size_t j = i + 1; j < result.size(); j++) {
      double distance = (result[i] - result[j]).norm();
      if (distance < min_distance) {
        min_distance = distance;
      }
    }
  }

  CHECK(min_distance == doctest::Approx(1));
}

TEST_CASE("testing multiple walk sarw") {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vec3<double>(0, 0, 0), 10);

  molpack::SARW sarw(std::move(region), 1);

  std::vector<molpack::Vec3<double>> result;

  int n = 10;
  for (int i = 0; i < n; i++) {
    auto walk_result = sarw.walk(100);
    result.insert(result.end(), walk_result.begin(), walk_result.end());

    CHECK(walk_result.size() == 100);
  }

  double min_distance = 100;

  for (std::size_t i = 0; i < result.size(); i++) {
    for (std::size_t j = i + 1; j < result.size(); j++) {
      double distance = (result[i] - result[j]).norm();
      if (distance < min_distance) {
        min_distance = distance;
      }
    }
  }

  CHECK(min_distance == doctest::Approx(1));
}