#ifndef MOLPACK_GEOM_H
#define MOLPACK_GEOM_H

#include <vector>

#include "vec3.hpp"

namespace molpack {

Vec3<double>& translate(Vec3<double>& v, Vec3<double>& t);

Vec3<double>& rotate(Vec3<double>& v, Mat<double> rot);

Vec3<double>& rotate(Vec3<double>& v, Vec3<double>& axis, double theta);

Vec3<double>& rotate(Vec3<double>& v, double alpha, double beta, double gamma);

}  // namespace molpack

#endif  // MOLPACK_GEOM_H