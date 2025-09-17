#include "max.hxx"
#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <ranges>
#include <span>


inline auto expandIotaViews(std::span<int32_t> input) {
    auto iota_transform = [](const int number) { return std::views::iota(1, number + 1); };

    return input | std::views::transform(iota_transform) | std::views::join | std::views::transform(iota_transform) | std::views::join | std::views::transform(iota_transform) | std::views::join;
}

extern "C" int32_t cpp_max_ptr(int32_t* ptr, size_t len) noexcept {
    std::span<int32_t> s = {ptr, len};
    auto result = expandIotaViews(s);
    return std::ranges::max(result);
}
