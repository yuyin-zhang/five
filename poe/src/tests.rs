use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;


// 为测试的test定义了一个Origin表示测试的发送方
impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64; // AccuntId u64
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	// 设置的存证的长度最大为2
	pub const ClaimLength : usize = 128;
}

impl Trait for Test {
	type Event = (); // 空的元组默认实现了这个event的关联类型的约束
	type ClaimLength = ClaimLength;
}

pub type PoeModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities { // 返回了一个测试用的执行环境
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}


use crate::{Error, mock::*, Trait};
use frame_support::{assert_ok, assert_noop};
use super::*;



// 测试存证创建成功
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()))
    })
}

// 创建存证失败，因为已经有一个同名的存证存在
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!( // assert_noops! 不会修改链上的状态
            PoeModule::create_claim(Origin::signed(1), claim.clone()), // 断言生成的是error
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 测试吊销存证成功
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

// 吊销存证但是存证不存在
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::CalimNotExist
        );
    })
}

// 吊销存证但是不是交易的发送方
#[test]
fn revoke_claim_failed_when_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移存证成功
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _  = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 23u64));

        assert_eq!(Proofs::<Test>::get(&claim), (23, frame_system::Module::<Test>::block_number()));

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移存证，但是转移的发起者不是交易的发送方
#[test]
fn transfer_claim_failed_when_is_transfer_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 23),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移的存证数据不存在
#[test]
fn transfer_claim_failed_when_claim_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim_temp = vec![2, 3];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim_temp.clone(), 23),
            Error::<Test>::CalimNotExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_length_is_too_large() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimLengthTooLarge,
        );
    })
}

