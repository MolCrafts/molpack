#include <iostream>

#include "region.h"
#include "sarw.h"
int main(int argc, char** argv) {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vector3D<double>(0, 0, 0), 10);

  molpack::SARW sarw(std::move(region), 1);

  auto result = sarw.walk(100);

  for (const auto& point : result) {
    std::cout << point.getX() << " " << point.getY() << " " << point.getZ()
              << std::endl;
  }

  return 0;
}