## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **11** | 🟥 271 (+260) | 🟩 12 (+1) |
| log | 🟩 **117** | 🟥 375 (+258) | 🟩 119 (+2) |
| create_account | 🟩 **1612** | 🟥 4428 (+2816) | 🟩 1791 (+179) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **16672** | 🟥 192928 (+176256) | 🟩 18496 (+1824) |
