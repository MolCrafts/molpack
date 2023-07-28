#ifndef MOLPACK_vec3_H_
#define MOLPACK_vec3_H_
#include <cmath>
namespace molpack {
template <typename T>
class Vec3 {
 private:
  T inner_[3]{};

 public:
  Vec3<T>() = default;
  Vec3<T>(const T& x, const T& y, const T& z) : inner_{x, y, z} {}

  inline const T& getX() const { return inner_[0]; }
  inline const T& getY() const { return inner_[1]; }
  inline const T& getZ() const { return inner_[2]; }

  inline void setX(T new_val) { inner_[0] = new_val; }
  inline void setY(T new_val) { inner_[1] = new_val; }
  inline void setZ(T new_val) { inner_[2] = new_val; }

  template <typename U>
  inline friend Vec3<decltype(T{} + U{})> operator+(
      const Vec3<T>& lhs, const Vec3<U>& rhs) {
    return {lhs.getX() + rhs.getX(), lhs.getY() + rhs.getY(),
            lhs.getZ() + rhs.getZ()};
  }

  template <typename U>
  inline friend Vec3<decltype(T{} - U{})> operator-(
      const Vec3<T>& lhs, const Vec3<U>& rhs) {
    return {lhs.getX() - rhs.getX(), lhs.getY() - rhs.getY(),
            lhs.getZ() - rhs.getZ()};
  }

  template <typename U>
  inline Vec3<decltype(T{} * U{})> operator*(const U& rhs) const {
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