#ifndef MOLPACK_NEIGHBOR_LIST_H_
#define MOLPACK_NEIGHBOR_LIST_H_

#include <algorithm>
#include <cmath>
#include <cstddef>
#include <deque>
#include <functional>
#include <iterator>
#include <limits>
#include <utility>
#include <vector>

#include "vector3d.hpp"

namespace molpack {
template <typename T>
class CellList {
 public:
  using value_type = T;
  static constexpr std::size_t EMPTY = std::numeric_limits<std::size_t>::max();

 private:
  Vector3D<double> edge_lengths_;
  Vector3D<double> origin_;
  Vector3D<std::size_t> num_cells_;

  std::deque<std::pair<Vector3D<double>, value_type>> contents_{};
  std::vector<std::size_t> head_;
  std::deque<std::size_t> next_{};

  static Vector3D<double> calcEdgeLengths(Vector3D<double> lengths,
                                          Vector3D<double> cutoff_lengths) {
    Vector3D<double> edge_lengths(
        lengths.getX() / std::floor(lengths.getX() / cutoff_lengths.getX()),
        lengths.getY() / std::floor(lengths.getY() / cutoff_lengths.getY()),
        lengths.getZ() / std::floor(lengths.getZ() / cutoff_lengths.getZ()));
    return edge_lengths;
  }

  static Vector3D<std::size_t> calcNumCells(Vector3D<double> lengths,
                                            Vector3D<double> cutoff_lengths) {
    return Vector3D<std::size_t>(
        static_cast<std::size_t>(
            std::floor(lengths.getX() / cutoff_lengths.getX())),
        static_cast<std::size_t>(
            std::floor(lengths.getY() / cutoff_lengths.getY())),
        static_cast<std::size_t>(
            std::floor(lengths.getZ() / cutoff_lengths.getZ())));
  }

  Vector3D<std::size_t> calcCellIdVec(Vector3D<double> position) {
    return Vector3D<std::size_t>(
        static_cast<std::size_t>(std::floor((position.getX() - origin_.getX()) /
                                            edge_lengths_.getX())),
        static_cast<std::size_t>(std::floor((position.getY() - origin_.getY()) /
                                            edge_lengths_.getY())),
        static_cast<std::size_t>(std::floor((position.getZ() - origin_.getZ()) /
                                            edge_lengths_.getZ())));
  }

  std::size_t calcCellId(Vector3D<std::size_t> cell_idvec) {
    return cell_idvec.getX() + cell_idvec.getY() * this->num_cells_.getX() +
           cell_idvec.getZ() * this->num_cells_.getX() *
               this->num_cells_.getY();
  }

  std::size_t calcCellId(std::size_t x, std::size_t y, std::size_t z) {
    return x + y * this->num_cells_.getX() +
           z * this->num_cells_.getX() * this->num_cells_.getY();
  }

 public:
  CellList(Vector3D<double> origin, Vector3D<double> lengths,
           Vector3D<double> cutoff_lengths)
      : edge_lengths_(calcEdgeLengths(lengths, cutoff_lengths)),
        origin_(origin),
        num_cells_(calcNumCells(lengths, cutoff_lengths)),
        head_(this->num_cells_.product(), EMPTY) {}

  CellList(Vector3D<double> origin, Vector3D<double> lengths,
           double cutoff_length)
      : CellList(
            origin, lengths,
            Vector3D<double>(cutoff_length, cutoff_length, cutoff_length)) {}
  ~CellList() = default;

  std::size_t size() const { return contents_.size(); }

  const std::pair<Vector3D<double>, value_type> &operator[](int index) const {
    return contents_[index];
  }

  void append(Vector3D<double> position, value_type val) {
    std::size_t id = this->size();
    contents_.emplace_back(position, val);

    auto cell_idvec = this->calcCellIdVec(position);
    auto cell_id = this->calcCellId(cell_idvec);

    if (this->head_[cell_id] == EMPTY) {
      this->head_[cell_id] = id;
      this->next_.emplace_back(EMPTY);
    } else {
      this->next_.emplace_back(this->head_[cell_id]);
      this->head_[cell_id] = id;
    }
  }

  void foreachNeighbor(
      Vector3D<double> position,
      std::function<bool(const std::pair<Vector3D<double>, value_type> &)>
          func) {
    auto cell_idvec = this->calcCellIdVec(position);

    // foreach the neighbor cells and avoid exceeding the boundary
    auto min_i = std::max(cell_idvec.getX(), static_cast<std::size_t>(1)) - 1;
    auto max_i = std::min(cell_idvec.getX() + 1, this->num_cells_.getX() - 1);
    auto min_j = std::max(cell_idvec.getY(), static_cast<std::size_t>(1)) - 1;
    auto max_j = std::min(cell_idvec.getY() + 1, this->num_cells_.getY() - 1);
    auto min_k = std::max(cell_idvec.getZ(), static_cast<std::size_t>(1)) - 1;
    auto max_k = std::min(cell_idvec.getZ() + 1, this->num_cells_.getZ() - 1);

    for (auto i = min_i; i <= max_i; ++i) {
      for (auto j = min_j; j <= max_j; ++j) {
        for (auto k = min_k; k <= max_k; ++k) {
          auto cell_id = this->calcCellId(i, j, k);
          auto current_content_id = this->head_[cell_id];
          while (current_content_id != EMPTY) {
            // the func return true means the loop should be stopped
            if (func(this->contents_[current_content_id])) {
              return;
            }
            current_content_id = this->next_[current_content_id];
          }
        }
      }
    }
  }
  // value_type &operator[](int index) { return contents_[index]; }

  // class Iterator {
  //  public:
  //   using iterator_category = std::forward_iterator_tag;
  //   using difference_type = std::ptrdiff_t;
  //   using value_type = std::pair<Vector3D<double>, T>;
  //   using pointer = value_type *;
  //   using reference = value_type &;

  //  private:
  //   pointer ptr_;

  //  public:
  //   Iterator(pointer ptr) : ptr_(ptr) {}

  //   reference operator*() const { return *ptr_; }
  //   pointer operator->() const { return ptr_; }

  //   Iterator &operator++() {
  //     ++ptr_;
  //     return *this;
  //   }

  //   Iterator operator++(int) {
  //     Iterator tmp = *this;
  //     ++ptr_;
  //     return tmp;
  //   }

  //   friend bool operator==(const Iterator &a, const Iterator &b) {
  //     return a.ptr_ == b.ptr_;
  //   }

  //   friend bool operator!=(const Iterator &a, const Iterator &b) {
  //     return a.ptr_ != b.ptr_;
  //   }
  // };

  // Iterator begin() { return Iterator(&contents_[0]); }
  // Iterator end() { return Iterator(&contents_[contents_.size()]); }
};
}  // namespace molpack
#endif
