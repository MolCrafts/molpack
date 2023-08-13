#include "structure.hpp"

namespace molpack
{

    Structure::Structure()
    {
    }

    Structure::~Structure()
    {
    }

    Structure::addAtom(const Vec3<double>& xyz)
    {
        atoms_.push_back(xyz);
    }

    Structure::addBond(size_t i, size_t j)
    {
        bonds_.push_back(std::make_pair(i, j));
    }

    Structure::translate(const Vec3<double>& xyz)
    {
        for (auto& xyz : xyz_)
        {
            xyz += xyz;
        }
    }

    Structure::rotate(const Vec3<double>& axis, double angle)
    {

    }


} // namespace molpack