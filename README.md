# Bitcoin to Solana

솔라나 스마트 컨트랙트(프로그램) 학습 프로젝트입니다. [Anchor](https://www.anchor-lang.com/) 프레임워크를 사용해 간단한 카운터 프로그램을 구현하고, [LiteSVM](https://github.com/LiteSVM/litesvm)으로 온체인 배포 없이 로컬에서 테스트합니다.

---

## 목차

- [프로젝트 개요](#프로젝트-개요)
- [프로그램 명세](#프로그램-명세)
- [프로젝트 구조](#프로젝트-구조)
- [환경 요구사항](#환경-요구사항)
- [빌드 및 테스트](#빌드-및-테스트)
- [아키텍처](#아키텍처)
- [테스트 전략](#테스트-전략)

---

## 프로젝트 개요

| 항목 | 값 |
|------|-----|
| 프레임워크 | Anchor `1.0.2` |
| Rust 버전 | `1.89.0` (stable) |
| 프로그램 ID | `4Cfgg3SY8iWJm6YmEbXZpAvmnghPbGKfPT8HBG3TizX2` |
| 테스트 런타임 | LiteSVM `0.10.0` |
| 클러스터 | localnet |

---

## 프로그램 명세

카운터 값을 온체인에 저장하고 증가시키는 두 가지 명령어(instruction)를 제공합니다.

### `initialize`

새로운 `Counter` 계정을 생성하고 `count`를 `0`으로 초기화합니다.

| 계정 | 타입 | 역할 |
|------|------|------|
| `counter` | `Account<Counter>` (init) | 새로 생성할 카운터 저장 계정 (8 + 8 bytes) |
| `user` | `Signer` (mut) | 계정 생성 수수료를 지불하는 지갑 |
| `system_program` | `Program<System>` | 솔라나 시스템 프로그램 |

### `increment`

기존 `Counter` 계정의 `count` 값을 1 증가시킵니다.

| 계정 | 타입 | 역할 |
|------|------|------|
| `counter` | `Account<Counter>` (mut) | 증가시킬 카운터 저장 계정 |

### `Counter` 계정 구조

```rust
#[account]
pub struct Counter {
    pub count: u64,  // 64비트 부호없는 정수 (8 bytes)
}
```

---

## 프로젝트 구조

```
bitcoin-to-solana/
├── Anchor.toml                    # Anchor 설정 (클러스터, 지갑 경로, 프로그램 ID)
├── Cargo.toml                     # Cargo 워크스페이스 루트
├── rust-toolchain.toml            # Rust 툴체인 고정 (1.89.0)
├── migrations/
│   └── deploy.ts                  # Anchor 배포 스크립트
└── programs/
    └── test_project/
        ├── Cargo.toml             # 프로그램 의존성 (anchor-lang, litesvm 등)
        ├── src/
        │   ├── lib.rs             # 프로그램 진입점 — 명령어 핸들러, 계정 구조체, Counter 정의
        │   ├── constants.rs       # 프로그램 상수 (SEED)
        │   ├── error.rs           # 커스텀 에러 코드
        │   ├── state.rs           # 계정 데이터 구조체 (확장용 스캐폴드)
        │   ├── instructions.rs    # 명령어 모듈 배럴 파일
        │   └── instructions/
        │       └── initialize.rs  # Initialize 명령어 스텁
        └── tests/
            └── test_initialize.rs # LiteSVM 통합 테스트
```

> 현재 핵심 로직은 모두 `lib.rs`에 위치합니다. `instructions/` 와 `state.rs`는 향후 리팩토링을 위한 스캐폴드입니다.

---

## 환경 요구사항

- **Rust** `1.89.0` — `rust-toolchain.toml`로 자동 고정
- **Solana CLI** — `cargo build-sbf` 실행에 필요
- **Anchor CLI** `1.0.2`

---

## 빌드 및 테스트

> **중요:** 테스트는 컴파일된 `.so` 바이너리를 `include_bytes!`로 직접 로드하므로, **반드시 테스트 전에 빌드를 먼저 실행해야 합니다.**

```bash
# 1. 솔라나 프로그램 컴파일 (.so 바이너리 생성)
cargo build-sbf

# 2. 전체 테스트 실행
cargo test

# 특정 테스트만 실행
cargo test test_initialize

# 린트
cargo clippy

# 코드 포맷
cargo fmt
```

---

## 아키텍처

Anchor 프로그램은 세 가지 영역으로 구성됩니다.

```
┌─────────────────────────────────────────────┐
│              lib.rs                         │
│                                             │
│  1. #[program]  — 비즈니스 로직              │
│     initialize() / increment()              │
│                                             │
│  2. #[derive(Accounts)]  — 계정 검증         │
│     Initialize / Increment 구조체            │
│                                             │
│  3. #[account]  — 온체인 데이터 스키마        │
│     Counter { count: u64 }                  │
└─────────────────────────────────────────────┘
```

솔라나 프로그램의 데이터 모델은 일반 백엔드와 다음과 같이 대응됩니다.

| 솔라나 | 일반 백엔드 |
|--------|------------|
| `#[program]` 모듈 | 컨트롤러 / 서비스 레이어 |
| `#[derive(Accounts)]` 구조체 | 요청 유효성 검사 (DTO) |
| `#[account]` 구조체 | DB 스키마 |
| `Counter` 계정 | DB 레코드 (영구 저장) |

---

## 테스트 전략

[LiteSVM](https://github.com/LiteSVM/litesvm)을 사용해 풀 노드 없이 인프라 없는 통합 테스트를 수행합니다.

```
테스트 흐름:

LiteSVM 인스턴스 생성
    └─▶ .so 바이너리 로드 (include_bytes!)
    └─▶ 페이어 계정에 SOL 에어드롭
    └─▶ Initialize 명령어 트랜잭션 구성
    └─▶ VersionedTransaction 전송
    └─▶ 결과 검증 (assert!)
```

LiteSVM은 솔라나 런타임을 프로세스 내부에서 실행하므로, 실제 트랜잭션 처리 로직을 로컬 밸리데이터나 devnet 없이 테스트할 수 있습니다.
