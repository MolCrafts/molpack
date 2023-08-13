#include "geom.hpp"
#include "vec3.hpp"

namespace molpack
{

    Vec3<double>& translate(Vec3<double> &v, Vec3<double> &t)
    {
        v += t;

        return v;
    }

    Vec3<double>& rotate(Vec3<double> &v, Mat<double> &rot)
    {
        return ;
    }

    Vec3<double>& rotate(Vec3<double> &v, Vec3<double> &axis, double theta)
    {

    }

}