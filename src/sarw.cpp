#include "sarw.hpp"

#include <cmath>
#include <deque>
#include <optional>
#include <random>

#include "cell_list.hpp"
#define M_PI 3.14159265358979323846 /* pi */
namespace molpack {
constexpr double EPS = 1e-6;
bool SARW::hasNearby(Vec3<double> position, double cutoff_length,
                     std::optional<Vec3<double>> exclude_position) {
  bool has_nearby = false;
  this->cell_list_.foreachNeighbor(position, [&](const auto& neighbor) {
    if (exclude_position.has_value() &&
        (neighbor.first - exclude_position.value()).norm() < EPS) {
      return false;
    }
    if ((neighbor.first - position).norm() < cutoff_length) {
      has_nearby = true;
      return true;
    }
    return false;
  });
  return has_nearby;
}

std::deque<Vec3<double>> SARW::walk(std::size_t num_steps) {
  std::deque<Vec3<double>> result;
  double temperature = 5.0;

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
  auto gen_start_point = [&, this]() {
    while (true) {
      Vec3<double> start_point(dis_x(gen), dis_y(gen), dis_z(gen));
      if (this->region_->isInside(start_point) &&
          !this->hasNearby(start_point, this->step_length_, std::nullopt)) {
        this->cell_list_.append(start_point, 0);
        result.emplace_back(start_point);
        break;
      }
    }
  };
  gen_start_point();

  while (result.size() < num_steps) {
    // check if the temperature is too low
    if (temperature < 1.0) {
      // undo the last step
      this->cell_list_.pop_back();
      result.pop_back();

      temperature += 0.5;

      if (result.size() == 0) {
        // randomly choose a starting point
        gen_start_point();
      }
      continue;
    }

    const auto& current_point = result.back();

    // randomly choose a direction
    double theta = dis_theta(gen);
    double phi = dis_phi(gen);

    // move to the next point
    auto next_point =
        current_point + Vec3<double>(this->step_length_ * sin(phi) * cos(theta),
                                     this->step_length_ * sin(phi) * sin(theta),
                                     this->step_length_ * cos(phi));

    // check if the next point is inside the region
    if (!this->region_->isInside(next_point)) {
      temperature *= 0.95;
      continue;
    }

    // check if the next point is too close to the previous points
    if (this->hasNearby(next_point, this->step_length_, current_point)) {
      temperature *= 0.9;
      continue;
    }

    temperature += 0.25;
    this->cell_list_.append(next_point, 0);
    result.emplace_back(next_point);
  }

  return result;
}

}  // namespace molpack