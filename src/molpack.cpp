#include <iostream>

#include "region.hpp"
#include "sarw.hpp"
int main(int argc, char** argv) {
  auto region = std::make_unique<molpack::SphericalRegion>(
      molpack::Vector3D<double>(0, 0, 0), 10);

  molpack::SARW sarw(std::move(region), 1);

  int n = 10;
  std::cout << n << std::endl;
  for (int i = 0; i < n; i++) {
    auto result = sarw.walk(100);
    std::cout << result.size() << std::endl;

    for (const auto& point : result) {
      std::cout << point.getX() << " " << point.getY() << " " << point.getZ()
                << std::endl;
    }
  }

  return 0;
}