#include "sarw.h"

#include <deque>
#include <random>

namespace molpack {

std::deque<Vector3D<double>> SARW::walk(int num_steps) {
  std::deque<Vector3D<double>> result;

  // initialize random number generator
  std::random_device rd;
  std::mt19937 gen(rd());

  // randomly choose a starting point
  auto region_size = this->region_->size();
  auto region_origin = this->region_->origin();

  std::uniform_real_distribution<> dis_x(
      region_origin.getX(), region_size.getX() + region_origin.getX());
  std::uniform_real_distribution<> dis_y(
      region_origin.getY(), region_size.getY() + region_origin.getY());
  std::uniform_real_distribution<> dis_z(
      region_origin.getZ(), region_size.getZ() + region_origin.getZ());

  while (true) {
    Vector3D<double> start_point(dis_x(gen), dis_y(gen), dis_z(gen));
    if (this->region_->isInside(start_point)) {
      result.emplace_back(start_point);
      break;
    }
  }

  std::uniform_real_distribution<> dis_theta(0, 2 * M_PI);
  std::uniform_real_distribution<> dis_phi(0, M_PI);
  while (result.size() < num_steps) {
    const auto& current_point = result.back();

    // randomly choose a direction
    double theta = dis_theta(gen);
    double phi = dis_phi(gen);

    // move to the next point
    auto next_point =
        current_point +
        Vector3D<double>(this->step_length_ * sin(phi) * cos(theta),
                         this->step_length_ * sin(phi) * sin(theta),
                         this->step_length_ * cos(phi));

    // check if the next point is inside the region
    if (this->region_->isInside(next_point)) {
      result.emplace_back(next_point);
    }
  }

  return result;
}

}  // namespace molpack