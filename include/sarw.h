#ifndef MOLPACK_SARW_H_
#define MOLPACK_SARW_H_

#include <memory>
#include <utility>
#include <deque>

#include "region.h"
#include "vector3d.h"
namespace molpack {
class SARW {
 private:
  std::unique_ptr<Region> region_;
  double step_length_;

 public:
  SARW(std::unique_ptr<Region> region, double step_length)
      : region_(std::move(region)), step_length_(step_length) {}

  std::deque<Vector3D<double>> walk(int num_steps);
};

}  // namespace molpack

#endif