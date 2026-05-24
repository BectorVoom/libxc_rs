//! MGGA_C_RMGGAC lxc_pol shard mgga_c_rmggac_p0 — thin module index (no cube wrapper; the wrapper lives in the facade).
//! Parts 0..=11.

pub mod part0;
pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;
pub mod part5;
pub mod part6;
pub mod part7;
pub mod part8;
pub mod part9;
pub mod part10;
pub mod part11;

pub use part0::mgga_c_rmggac_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
pub use part1::mgga_c_rmggac_lxc_pol_part1_v3rho3;
pub use part2::mgga_c_rmggac_lxc_pol_part2_v3rho2sigma_v3rho2lapl;
pub use part3::mgga_c_rmggac_lxc_pol_part3_v3rho2tau;
pub use part4::mgga_c_rmggac_lxc_pol_part4_v3rhosigma2_v3rhosigmalapl;
pub use part5::mgga_c_rmggac_lxc_pol_part5_v3rhosigmatau_v3rholapl2_v3rholapltau;
pub use part6::mgga_c_rmggac_lxc_pol_part6_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
pub use part7::mgga_c_rmggac_lxc_pol_part7_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3;
pub use part8::mgga_c_rmggac_lxc_pol_part8_v4rho4;
pub use part9::mgga_c_rmggac_lxc_pol_part9_v4rho3sigma_0;
pub use part10::mgga_c_rmggac_lxc_pol_part10_v4rho3sigma_1;
pub use part11::mgga_c_rmggac_lxc_pol_part11_v4rho3sigma_2;
