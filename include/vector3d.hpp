#ifndef MOLPACK_VECTOR3D_H_
#define MOLPACK_VECTOR3D_H_
#include <cmath>
namespace molpack {
template <typename T>
class Vector3D {
 private:
  T inner_[3]{};

 public:
  Vector3D<T>() = default;
  Vector3D<T>(const T& x, const T& y, const T& z) : inner_{x, y, z} {}

  inline const T& getX() const { return inner_[0]; }
  inline const T& getY() const { return inner_[1]; }
  inline const T& getZ() const { return inner_[2]; }

  inline void setX(T new_val) { inner_[0] = new_val; }
  inline void setY(T new_val) { inner_[1] = new_val; }
  inline void setZ(T new_val) { inner_[2] = new_val; }

  template <typename U>
  inline friend Vector3D<decltype(T{} + U{})> operator+(
      const Vector3D<T>& lhs, const Vector3D<U>& rhs) {
    return {lhs.getX() + rhs.getX(), lhs.getY() + rhs.getY(),
            lhs.getZ() + rhs.getZ()};
  }

  template <typename U>
  inline friend Vector3D<decltype(T{} - U{})> operator-(
      const Vector3D<T>& lhs, const Vector3D<U>& rhs) {
    return {lhs.getX() - rhs.getX(), lhs.getY() - rhs.getY(),
            lhs.getZ() - rhs.getZ()};
  }

  template <typename U>
  inline Vector3D<decltype(T{} * U{})> operator*(const U& rhs) const {
    return {inner_[0] * rhs, inner_[1] * rhs, inner_[2] * rhs};
  }

  inline double norm2() const {
    return std::sqrt(inner_[0] * inner_[0] + inner_[1] * inner_[1] +
                     inner_[2] * inner_[2]);
  }

  inline T product() const { return inner_[0] * inner_[1] * inner_[2]; }
};
}  // namespace molpack
#endif