#ifndef MOLPACK_SARW_H_
#define MOLPACK_SARW_H_

#include <deque>
#include <memory>
#include <optional>
#include <utility>

#include "cell_list.hpp"
#include "region.hpp"
#include "vec3.hpp"
namespace molpack {
class SARW {
 private:
  std::unique_ptr<Region> region_;
  double step_length_;
  CellList<int> cell_list_;

  bool hasNearby(Vec3<double> position, double cutoff_length,
                 std::optional<Vec3<double>> exclude_position);

 public:
  SARW(std::unique_ptr<Region> region, double step_length)
      : region_(std::move(region)),
        step_length_(step_length),
        cell_list_(this->region_->origin(), this->region_->size(),
                   step_length) {}

  std::deque<Vec3<double>> walk(std::size_t num_steps);
};

}  // namespace molpack

#endif