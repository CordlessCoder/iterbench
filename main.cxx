#include <algorithm>
#include <chrono>
#include <iostream>
#include <numeric>
#include <ostream>
#include <ranges>
#include <span>
#include <vector>


inline auto expandIotaViews(std::span<int32_t> input) {
    auto iota_transform = [](const int number) { return std::views::iota(1, number + 1); };

    return input | std::views::transform(iota_transform) | std::views::join | std::views::transform(iota_transform) | std::views::join | std::views::transform(iota_transform) | std::views::join;
}

int main() {
    for (int sample = 0; sample < 50; sample++) {
        std::vector<int> input(50);
        std::iota(input.begin(), input.end(), 0);

        auto sample_result = expandIotaViews(input);
        std::vector<int> result_vec;
        for (auto val : sample_result) {
            result_vec.push_back(val);
        }

        std::cout << "C++ Result count: " << result_vec.size() << std::endl;


        auto start = std::chrono::high_resolution_clock::now();
        size_t total_count = 0;
        for (int i = 0; i < 1000; ++i) {
            auto result = expandIotaViews(input);
            total_count += std::ranges::max(result);
        }
        auto end = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        std::cout << "C++ Total count (1000 iterations): " << total_count << std::endl;
        std::cout << "C++ Total time: " << duration.count() << " microseconds" << std::endl;
        std::cout << "C++ Average per iteration: " << duration.count() / 1000.0 << " microseconds" << std::endl;
    }

    return 0;
}
