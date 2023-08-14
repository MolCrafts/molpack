#include "structure.hpp"

namespace molpack
{

    Structure::Structure()
    {
    }

    Structure::Structure(const Structure& other)
    {
        xyz_ = other.xyz_;
        bonds_ = other.bonds_;
    }

    Structure::~Structure()
    {
    }

    void Structure::addAtom(const Vec3<double>& xyz)
    {
        xyz_.push_back(xyz);
    }

    void Structure::addBond(size_t i, size_t j)
    {
        bonds_.push_back(std::make_pair(i, j));
    }

    Structure& Structure::translate(const Vec3<double>& xyz)
    {
        for (auto& x : xyz_)
        {
            x = molpack::translate(x, xyz);
        }
        return *this;
    }

    Structure& Structure::rotate(const Vec3<double>& axis, double angle)
    {
        for (auto& x : xyz_)
        {
            x = molpack::rotate(x, axis, angle);
        }
        return *this;
    }

    Structure& Structure::rotate(double alpha, double beta, double gamma)
    {
        for (auto& x : xyz_)
        {
            x = molpack::rotate(x, alpha, beta, gamma);
        }
        return *this;
    }

    Structure& Structure::move_to(const Vec3<double>& xyz)
    {
        Vec3<double> center = getCenterOfMass();
        auto r = xyz - center;
        for (auto& x : xyz_)
        {
            x = molpack::translate(x, r);
        }
        return *this;
    }

    Structure Structure::clone() const
    {
        Structure new_struct(*this);
        return new_struct;
    }

    Vec3<double> Structure::getCenterOfMass() const
    {
        Vec3<double> center;
        for (auto& x : xyz_)
        {
            center += x;
        }
        center /= xyz_.size();
        return center;
    }

} // namespace molpack