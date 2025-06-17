## Benchmark Results

### Color Legend

- 🟩 **Green**: Best performance (minimum value) or within 50% of the best
- 🟨 **Yellow**: Moderate performance (up to 2x the minimum value)
- 🟥 **Red**: Poor performance (more than 2x the minimum value)

### CU Consumed

| Benchmark     | `pinocchio`     | `anchor`          | `typhoon`    |
| ------------- | --------------- | ----------------- | ------------ |
| ping | 🟩 **10** | 🟥 271 (+261) | 🟩 12 (+2) |
| log | 🟩 **117** | 🟥 375 (+258) | 🟩 119 (+2) |
| create_account | 🟩 **1611** | 🟥 4428 (+2817) | 🟩 1791 (+180) |

### Binary Size

|                     | `pinocchio`     | `anchor`            | `typhoon`|
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟩 **16672** | 🟥 192912 (+176240) | 🟩 18496 (+1824) |
