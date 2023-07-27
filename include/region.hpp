#ifndef MOLPACK_REGION_H_
#define MOLPACK_REGION_H_
#include "vector3d.hpp"
namespace molpack {
class Region {
public:
  virtual bool isInside(double x, double y, double z) const = 0;
  virtual bool isInside(Vector3D<double> coord) const {
    return isInside(coord.getX(), coord.getY(), coord.getZ());
  };
  virtual Vector3D<double> size() const = 0;
  virtual Vector3D<double> origin() const = 0;
  virtual ~Region() = default;
};

class SphericalRegion : public Region {
private:
  Vector3D<double> center_;
  double radius_;

public:
  SphericalRegion(Vector3D<double> center, double radius) : center_(center), radius_(radius) {}
  ~SphericalRegion() = default;

  bool isInside(double x, double y, double z) const override {
    return (x - center_.getX()) * (x - center_.getX()) +
               (y - center_.getY()) * (y - center_.getY()) +
               (z - center_.getZ()) * (z - center_.getZ()) <=
           radius_ * radius_;
  }

  Vector3D<double> size() const override {
    return Vector3D<double>(2 * radius_, 2 * radius_, 2 * radius_);
  }

  Vector3D<double> origin() const override {
    return Vector3D<double>(center_.getX() - radius_, center_.getY() - radius_,
                            center_.getZ() - radius_);
  }
};
}  // namespace molpack

#endif