# zkWasm Launchpad - IDO Platform

A comprehensive zkWasm-based Initial DEX Offering (IDO) launchpad platform supporting multi-project fundraising with dynamic token allocation, over-subscription handling, and USDT-based investments.

## 🚀 Features

### Core IDO Functions
- **Multi-Project Support**: Create and manage multiple IDO projects simultaneously
- **Dynamic Project Creation**: Admin can create IDO projects with custom parameters
- **USDT Investment**: Users invest using USDT through proxy contracts
- **Over-subscription Handling**: Dynamic proportional token distribution when over-subscribed
- **Time-based Management**: Automatic project status transitions based on timing
- **Individual Investment Caps**: Maximum investment limits per user per project

### Advanced Features
- **Dynamic Token Allocation**: Token amounts calculated dynamically based on total raised
- **Immediate Withdrawal**: Users can withdraw tokens immediately after project ends
- **Flexible Withdrawal**: USDT withdrawal always available, token withdrawal after deadline
- **Investment Tracking**: Complete investment history per user and project
- **Real-time Analytics**: Live project statistics and funding progress
- **Admin Controls**: Project management and fund administration

### Security & Safety
- **Mathematical Safety**: Comprehensive overflow/underflow protection using safe arithmetic
- **Input Validation**: Strict validation of all parameters and amounts
- **Time Validation**: Proper deadline and timing enforcement
- **Access Control**: Role-based permissions for admin vs user operations

## 🏗️ Technical Architecture

### Rust Backend (`src/`)
```
├── lib.rs                 # Application entry point and zkWasm API
├── config.rs              # Configuration constants and event types
├── error.rs               # Error code definitions and handling
├── event.rs               # Event system implementation
├── command.rs             # Transaction command processing
├── player.rs              # User data structures and operations
├── ido.rs                 # IDO project logic and token distribution
├── math_safe.rs           # Safe mathematical operations
├── settlement.rs          # Withdrawal settlement system
├── state.rs               # Global state and project management
└── security_tests.rs      # Comprehensive security tests
```

### TypeScript Backend (`ts/src/`)
```
├── service.ts             # Main service with REST API endpoints
├── models.ts              # Data models and MongoDB schemas
├── api.ts                 # Client API and transaction builders
├── test.ts                # Comprehensive IDO testing
├── create_ido.ts          # IDO creation scripts
├── deposit.ts             # USDT deposit scripts
└── sanity_sync.ts         # Sanity CMS integration (optional)
```

## 📊 IDO Distribution Algorithm

### Investment and Distribution Logic
- **Investment Phase**: Users deposit USDT, tracked per project
- **Dynamic Calculation**: Token allocation calculated in real-time based on current state
- **Over-subscription Handling**: 
  - If `total_raised <= target_amount`: Full allocation based on target
  - If `total_raised > target_amount`: Proportional allocation based on total raised

### Token Distribution Formula
```rust
// For over-subscribed projects
user_tokens = (user_investment * token_supply) / total_raised
allocation_ratio = target_amount / total_raised
refund_amount = user_investment * (1 - allocation_ratio)

// For normal projects
user_tokens = (user_investment * token_supply) / target_amount
refund_amount = 0
```

### Example Calculation
```typescript
// Project: Target 100,000 USDT for 1,000,000 tokens
// Total raised: 150,000 USDT (over-subscribed by 50%)

// User A invested 10,000 USDT
// User A allocation: (10,000 * 1,000,000) / 150,000 = 66,666 tokens
// Allocation ratio: 100,000 / 150,000 = 0.6667
// User A refund: 10,000 * (1 - 0.6667) = 3,333 USDT
```

## 🔌 API Endpoints

### Project Data
- `GET /data/idos` - Get all IDO projects
- `GET /data/ido/:projectId` - Get specific IDO project details
- `GET /data/ido/:projectId/investors` - Get project investor list
- `GET /data/ido/:projectId/stats` - Get project statistics

### User Data
- `GET /data/user/:pid1/:pid2/investments` - User's investments across all projects
- `GET /data/user/:pid1/:pid2/ido/:projectId` - User's investment in specific project
- `GET /data/user/:pid1/:pid2/withdrawals` - User's withdrawal history
- `GET /data/user/:pid1/:pid2/positions` - User's all project positions

### Platform Data
- `GET /data/platform/stats` - Platform-wide statistics

## 🎮 Transaction Commands

| Command ID | Command | Parameters | Permission | Description |
|------------|---------|------------|------------|-------------|
| 0 | TICK | - | System | Increment global counter and update project status |
| 1 | INSTALL_PLAYER | - | Any | Register new user |
| 2 | WITHDRAW_USDT | amount, addr_high, addr_low | User | Withdraw USDT to external address |
| 3 | DEPOSIT_USDT | target_pid1, target_pid2, amount | Admin | Deposit USDT for user via proxy |
| 4 | INVEST | project_id, amount | User | Invest USDT in IDO project |
| 5 | WITHDRAW_TOKENS | project_id | User | Withdraw allocated tokens (after project ends) |
| 6 | CREATE_IDO | project_data | Admin | Create new IDO project |
| 7 | UPDATE_IDO | project_id, update_data | Admin | Update IDO project parameters |

## 📡 Event System

### Event Types
- **EVENT_IDO_UPDATE (1)**: IDO project updates and status changes
- **EVENT_INVESTMENT (2)**: Investment transaction events
- **EVENT_WITHDRAWAL (3)**: Token/USDT withdrawal events
- **EVENT_PLAYER_UPDATE (4)**: Player balance and state updates

### IndexedObject Data
- **IDO_PROJECT_INFO (1)**: Complete IDO project state
- **INVESTMENT_HISTORY_INFO (2)**: Investment tracking data
- **USER_ALLOCATION_INFO (3)**: User token allocation data

## 💻 Data Structures

### IDO Project Structure
```rust
pub struct IdoProject {
    pub id: u64,
    pub name: Vec<u64>,                  // Project name (encoded)
    pub token_name: Vec<u64>,            // Token name (encoded)
    pub token_symbol: Vec<u64>,          // Token symbol (encoded)
    pub target_amount: u64,              // Target USDT amount
    pub token_supply: u64,               // Total tokens to distribute
    pub max_individual_cap: u64,         // Maximum investment per user
    pub start_time: u64,                 // Investment start time
    pub end_time: u64,                   // Investment deadline
    pub total_raised: u64,               // Current total raised
    pub investor_count: u64,             // Number of investors
    pub status: ProjectStatus,           // PENDING, ACTIVE, ENDED
    pub admin_address: [u64; 2],         // Project admin
    pub created_time: u64,               // Project creation time
}
```

### Project Status Enum
```rust
pub enum ProjectStatus {
    PENDING = 0,    // Created but not started
    ACTIVE = 1,     // Investment period active
    ENDED = 2,      // Investment period ended, tokens can be withdrawn
}
```

### User Investment Structure
```rust
pub struct UserInvestment {
    pub user_id: [u64; 2],
    pub project_id: u64,
    pub invested_amount: u64,            // USDT invested
    pub tokens_withdrawn: bool,          // Token withdrawal status
    pub refund_withdrawn: bool,          // Refund withdrawal status
    pub investment_time: u64,            // Investment timestamp
}
```

**Note**: Token allocation and refund amounts are calculated dynamically based on project state, not stored.

## 🚦 Project Lifecycle

### Phase 1: Project Creation
1. Admin creates IDO project with parameters
2. Project enters PENDING status
3. Parameters can be updated before start time

### Phase 2: Investment Period
1. Project becomes ACTIVE at start time (via TICK updates)
2. Users can invest USDT up to individual caps
3. Real-time tracking of total raised amount

### Phase 3: Project End and Token Distribution
1. Project ends at deadline (ENDED status via TICK updates)
2. Token allocations calculated dynamically based on total raised
3. Users can immediately withdraw allocated tokens and refunds
4. USDT withdrawals remain available throughout all phases

## 🔧 TypeScript API Usage

### Creating IDO Projects
```typescript
import { LaunchpadPlayer } from './api.js';

const admin = new LaunchpadPlayer(adminKey, rpc);

// Create new IDO project
await admin.createIdoProject(
    "DeFi Protocol Token",           // name
    "DeFi Protocol",                 // tokenName
    "DFP",                          // tokenSymbol
    1000000000000n,                 // targetAmount (1M USDT)
    10000000000000000000n,          // tokenSupply (10B tokens)
    100000000000n,                  // maxIndividualCap (100K USDT)
    120960n                         // durationTicks (1 week)
);
```

### User Investment Flow
```typescript
const user = new LaunchpadPlayer(userKey, rpc);

// Invest in IDO project
await user.investInProject(0n, 50000000000n); // Project 0, 50K USDT

// Withdraw tokens after project ends
await user.withdrawTokens(0n);

// Withdraw USDT to external address
await user.withdrawUsdt(
    10000000000n,           // amount
    0x1234567890abcdefn,    // addressHigh
    0xfedcba0987654321n     // addressLow
);
```

### API Data Retrieval
```typescript
import { IdoLaunchpadAPI } from './api.js';

const api = new IdoLaunchpadAPI();

// Get all projects
const projects = await api.getAllProjects();

// Get specific project
const project = await api.getProject("0");

// Get user investments
const investments = await api.getUserInvestments("123", "456");

// Calculate token allocation
const allocation = api.calculateTokenAllocation(
    userInvestment,
    totalRaised,
    tokenSupply,
    targetAmount,
    isOverSubscribed
);
```

## 🛡️ Security Features

### Mathematical Safety
- **Safe Arithmetic**: All operations use checked arithmetic to prevent overflow/underflow
- **Precision Handling**: 18-decimal precision for accurate calculations
- **Validation Functions**: Comprehensive input validation for all parameters

### Access Control
- **Admin Functions**: Project creation and management restricted to admin keys
- **User Functions**: Investment and withdrawal permissions per user
- **Time Enforcement**: Strict enforcement of project timing rules

### Investment Protection
- **Balance Verification**: Sufficient USDT balance checked before investment
- **Cap Enforcement**: Individual investment limits strictly enforced
- **Dynamic Calculation**: Token allocations calculated fresh on each withdrawal

## 🔍 Testing

The platform includes comprehensive tests:

### Rust Tests
```bash
cargo test
```
- Mathematical safety tests
- IDO allocation scenarios
- Security boundary tests
- Over-subscription handling
- Precision consistency tests

### TypeScript Tests
```bash
npm run build
node dist/test.js
```
- Full IDO lifecycle testing
- API endpoint testing
- Investment calculation testing
- Error handling verification

## 📈 Example Usage

### Complete IDO Flow
```typescript
// 1. Admin creates IDO project
await admin.createIdoProject(
    "GameFi Token Launch",
    "GameFi",
    "GFI",
    1000000000000n,     // 1M USDT target
    50000000000000000000n, // 50B tokens
    100000000000n,      // 100K USDT max per user
    120960n             // 1 week duration
);

// 2. Users invest during active period
await user1.investInProject(0n, 50000000000n); // 50K USDT
await user2.investInProject(0n, 75000000000n); // 75K USDT
await user3.investInProject(0n, 25000000000n); // 25K USDT

// 3. Project automatically ends at deadline (via TICK)

// 4. Users withdraw tokens (calculated dynamically)
await user1.withdrawTokens(0n);
await user2.withdrawTokens(0n);
await user3.withdrawTokens(0n);
```

This comprehensive launchpad platform provides a complete IDO solution with robust mathematical safety, dynamic token allocation, and secure fund management. The system supports multiple concurrent IDO projects while ensuring fair allocation and transparent distribution mechanisms.# zkwasm-launchpad
