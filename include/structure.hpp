#ifndef MOLPACK_STRUCT_H_
#define MOLPACK_STRUCT_H_

#include <vector>
#include <utility>

#include "vec3.hpp"
#include "geom.hpp"

namespace molpack
{

    class Structure
    {

        public:

        Structure();
        Structure(const Structure& other);
        ~Structure();

        void addAtom(const Vec3<double>& xyz);
        void addBond(size_t i, size_t j);

        Structure& translate(const Vec3<double>& xyz);
        Structure& rotate(const Vec3<double>& axis, double angle);
        Structure& rotate(double alpha, double beta, double gamma);

        Structure& move_to(const Vec3<double>& xyz);
        Structure clone() const;

        Vec3<double> getCenterOfMass() const;

        private:

        std::vector<Vec3<double>> xyz_;
        std::vector<std::pair<int, int>> bonds_;

    };

}

#endif