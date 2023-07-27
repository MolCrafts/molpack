#include "sarw.hpp"

#include <deque>
#include <random>

#include "cell_list.hpp"

namespace molpack {
constexpr double EPS = 1e-6;

std::deque<Vector3D<double>> SARW::walk(int num_steps) {
  std::deque<Vector3D<double>> result;

  // initialize random number generator and distributions
  std::random_device rd;
  std::mt19937 gen(rd());

  auto region_size = this->region_->size();
  auto region_origin = this->region_->origin();
  std::uniform_real_distribution<> dis_x(
      region_origin.getX(), region_size.getX() + region_origin.getX());
  std::uniform_real_distribution<> dis_y(
      region_origin.getY(), region_size.getY() + region_origin.getY());
  std::uniform_real_distribution<> dis_z(
      region_origin.getZ(), region_size.getZ() + region_origin.getZ());
  std::uniform_real_distribution<> dis_theta(0, 2 * M_PI);
  std::uniform_real_distribution<> dis_phi(0, M_PI);

  // randomly choose a starting point
  while (true) {
    Vector3D<double> start_point(dis_x(gen), dis_y(gen), dis_z(gen));
    if (this->region_->isInside(start_point)) {
      result.emplace_back(start_point);
      this->cell_list_.append(start_point, 0);
      break;
    }
  }

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
    if (!this->region_->isInside(next_point)) {
      continue;
    }

    // check if the next point is too close to the previous points
    bool too_close = false;
    this->cell_list_.foreachNeighbor(
        next_point,
        [&too_close, &next_point, &current_point, this](const auto& neighbor) {
          if ((neighbor.first - current_point).norm2() < EPS) {
            return false;
          }
          if ((neighbor.first - next_point).norm2() < this->step_length_) {
            too_close = true;
            return true;
          }
          return false;
        });

    if (too_close) {
      continue;
    }

    this->cell_list_.append(next_point, 0);
    result.emplace_back(next_point);
  }

  return result;
}

}  // namespace molpack