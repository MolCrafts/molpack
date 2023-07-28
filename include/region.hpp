#ifndef MOLPACK_REGION_H_
#define MOLPACK_REGION_H_
#include "vec3.hpp"
namespace molpack {
class Region {
public:
  virtual bool isInside(double x, double y, double z) const = 0;
  virtual bool isInside(Vec3<double> coord) const {
    return isInside(coord.getX(), coord.getY(), coord.getZ());
  };
  virtual Vec3<double> size() const = 0;
  virtual Vec3<double> origin() const = 0;
  virtual ~Region() = default;
};

class SphericalRegion : public Region {
private:
  Vec3<double> center_;
  double radius_;

public:
  SphericalRegion(Vec3<double> center, double radius) : center_(center), radius_(radius) {}
  ~SphericalRegion() = default;

  bool isInside(double x, double y, double z) const override {
    return (x - center_.getX()) * (x - center_.getX()) +
               (y - center_.getY()) * (y - center_.getY()) +
               (z - center_.getZ()) * (z - center_.getZ()) <=
           radius_ * radius_;
  }

  Vec3<double> size() const override {
    return Vec3<double>(2 * radius_, 2 * radius_, 2 * radius_);
  }

  Vec3<double> origin() const override {
    return Vec3<double>(center_.getX() - radius_, center_.getY() - radius_,
                            center_.getZ() - radius_);
  }
};
}  // namespace molpack

#endif