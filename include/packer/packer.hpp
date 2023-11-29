#ifndef MOLPACK_BASE_PACKER_HPP
#define MOLPACK_BASE_PACKER_HPP

#include <vector>

#include "structure.hpp"

namespace molpack {

    class Packer {

        public:

            virtual std::vector<Structure> pack(const Structure& structure, size_t num_struct) = 0;

        private:

    };

} // namespace molpack

#endif // MOLPACK_BASE_PACKER_HPP