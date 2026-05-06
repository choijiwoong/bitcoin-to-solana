pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

// 솔라나 스마트 컨트랙트 도구 임포트
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

// 블록체인 상에 배포될 고유 주소
declare_id!("4Cfgg3SY8iWJm6YmEbXZpAvmnghPbGKfPT8HBG3TizX2");

// 1. 비즈니스 로직 영역(스마트 컨트랙트 함수들)
#[program]
pub mod test_project {
    use super::*;

    // 함수 1. 새로운 카운터 저장소 생성 및 0 초기화
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 클라이언트가 보낸 계정정보목록(ctx)에서 counter 데이터 공간 가져옴
        // &mut는 mutable로 가변 변수 선언.
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        // 디버깅용 블록체인 트랜잭션 로그에 기록
        msg!("카운터 시작! 현재 값: {}", counter.count);
        // 함수가 에러 없이 끝남을 알림
        Ok(())
    }
    // 함수 2. 저장소에 기록된 숫자를 1씩 올림.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("카운터 증가! 현재 값: {}", counter.count);
        Ok(())
    }
}

// 2. 함수 실행 전 데이터 검증 및 선언
// 계정목록 정의
#[derive(Accounts)]
pub struct Initialize<'info> {
    // 상속 생성자 호출. user가 수수료를 내며, 데이터 저장은 16byte(기본+숫자저장용 공간)
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    // 트랜잭션 실행 및 수루료를 낼 사용자의 계정.
    #[account(mut)]
    pub user: Signer<'info>,

    // 솔라나에서 새 계정 생성 시 사용되는 기본 시스템 프로그램
    pub system_program: Program<'info, System>,
}

// incrediment함수 실행 전, 필요한 계정 목록 정의
#[derive(Accounts)]
pub struct Increment<'info> {
    // 기존의 counter 저장소를 가져옴
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// 3. DB 스키마 영역
// 블록체인 상 실제 영구 보존될 데이터 표현
#[account]
pub struct Counter {
    pub count: u64, //64비트 부호없는 정수(8byte)
}
