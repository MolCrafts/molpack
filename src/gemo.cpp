#include "geom.hpp"

namespace molpack
{

    Vec3<double>& translate(Vec3<double> &v, Vec3<double> &t)


    Vec3<double>& rotate(Vec3<double>& v, Mat3<double> rot)
    {
        v = rot * v;

        return v;
    }

    Vec3<double>& rotate(Vec3<double> &v, Vec3<double> &axis, double theta)
    {
        auto rot = Mat3<double>::zero();

        double c = cos(theta);
        double s = sin(theta);

        rot[0] = c + axis[0] * axis[0] * (1 - c);
        rot[1] = axis[0] * axis[1] * (1 - c) - axis[2] * s;
        rot[2] = axis[0] * axis[2] * (1 - c) + axis[1] * s;
        rot[3] = axis[1] * axis[0] * (1 - c) + axis[2] * s;
        rot[4] = c + axis[1] * axis[1] * (1 - c);
        rot[5] = axis[1] * axis[2] * (1 - c) - axis[0] * s;
        rot[6] = axis[2] * axis[0] * (1 - c) - axis[1] * s;
        rot[7] = axis[2] * axis[1] * (1 - c) + axis[0] * s;
        rot[8] = c + axis[2] * axis[2] * (1 - c);

        return rotate(v, rot);

    }

    Vec3<double>& rotate(Vec3<double>& v, double x, double y, double z, double alpha, double beta, double gamma)
    {
        auto rot = Mat3<double>::zero();

        double ca = cos(alpha);
        double sa = sin(alpha);
        double cb = cos(beta);
        double sb = sin(beta);
        double cg = cos(gamma);
        double sg = sin(gamma);

        rot[0] = ca * cb * cg - sa * sg;
        rot[1] = -ca * cb * sg - sa * cg;
        rot[2] = ca * sb;
        rot[3] = sa * cb * cg + ca * sg;
        rot[4] = -sa * cb * sg + ca * cg;
        rot[5] = sa * sb;
        rot[6] = -sb * cg;
        rot[7] = sb * sg;
        rot[8] = cb;

        return rotate(v, rot);
    }

}