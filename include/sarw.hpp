#ifndef MOLPACK_SARW_H_
#define MOLPACK_SARW_H_

#include <deque>
#include <memory>
#include <utility>

#include "cell_list.hpp"
#include "region.hpp"
#include "vector3d.hpp"
namespace molpack {
class SARW {
 private:
  std::unique_ptr<Region> region_;
  double step_length_;
  CellList<int> cell_list_;

 public:
  SARW(std::unique_ptr<Region> region, double step_length)
      : region_(std::move(region)),
        step_length_(step_length),
        cell_list_(this->region_->origin(), this->region_->size(),
                   step_length) {}

  std::deque<Vector3D<double>> walk(int num_steps);
};

}  // namespace molpack

#endif