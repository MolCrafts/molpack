#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "cell_list.hpp"

#include <memory>

#include "doctest/doctest.h"
#include "region.hpp"
#include "vec3.hpp"

TEST_CASE("testing cell_list append") {
  molpack::SphericalRegion region(molpack::Vec3<double>(0, 0, 0), 1);

  molpack::CellList<int> cell_list(region.origin(), region.size(), 0.1);

  cell_list.append(molpack::Vec3<double>(0.15, 0.15, 0.15), 0b1);
  cell_list.append(molpack::Vec3<double>(0.2, 0.2, 0.2), 0b10);
  cell_list.append(molpack::Vec3<double>(0.25, 0.25, 0.20), 0b100);
  cell_list.append(molpack::Vec3<double>(0.4, 0.4, 0.4), 0b1000);

  CHECK(cell_list.size() == 4);

  int sum = 0;
  cell_list.foreachNeighbor(molpack::Vec3<double>(0.2, 0.2, 0.2),
                            [&](std::pair<molpack::Vec3<double>, int> p) {
                              sum += p.second;
                              return false;
                            });
  CHECK(sum == 0b111);
}

TEST_CASE("testing cell_list append with negative value") {
  molpack::SphericalRegion region(molpack::Vec3<double>(0, 0, 0), 1);

  molpack::CellList<int> cell_list(region.origin(), region.size(), 0.1);

  cell_list.append(molpack::Vec3<double>(-0.15, -0.15, -0.15), 0b1);
  cell_list.append(molpack::Vec3<double>(-0.2, -0.2, -0.2), 0b10);
  cell_list.append(molpack::Vec3<double>(-0.25, -0.25, -0.20), 0b100);
  cell_list.append(molpack::Vec3<double>(-0.4, -0.4, -0.4), 0b1000);

  CHECK(cell_list.size() == 4);

  int sum = 0;
  cell_list.foreachNeighbor(molpack::Vec3<double>(-0.2, -0.2, -0.2),
                            [&](std::pair<molpack::Vec3<double>, int> p) {
                              sum += p.second;
                              return false;
                            });
  CHECK(sum == 0b111);
}