//! MGGA_C_TPSSLOC kxc pol kernel — kxc_pol (nested-by-output, 19 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;
mod part8;
mod part9;
mod part10;
mod part11;
mod part12;
mod part13;
mod part14;
mod part15;
mod part16;
mod part17;
mod part18;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_tpssloc_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::mgga_c_tpssloc_kxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;
use part2::mgga_c_tpssloc_kxc_pol_part2_v3rho3_0;
use part3::mgga_c_tpssloc_kxc_pol_part3_v3rho3_1;
use part4::mgga_c_tpssloc_kxc_pol_part4_v3rho3_2;
use part5::mgga_c_tpssloc_kxc_pol_part5_v3rho3_3;
use part6::mgga_c_tpssloc_kxc_pol_part6_v3rho2sigma_0;
use part7::mgga_c_tpssloc_kxc_pol_part7_v3rho2sigma_1;
use part8::mgga_c_tpssloc_kxc_pol_part8_v3rho2sigma_2;
use part9::mgga_c_tpssloc_kxc_pol_part9_v3rho2sigma_3;
use part10::mgga_c_tpssloc_kxc_pol_part10_v3rho2sigma_4;
use part11::mgga_c_tpssloc_kxc_pol_part11_v3rho2sigma_5;
use part12::mgga_c_tpssloc_kxc_pol_part12_v3rho2sigma_6;
use part13::mgga_c_tpssloc_kxc_pol_part13_v3rho2sigma_7;
use part14::mgga_c_tpssloc_kxc_pol_part14_v3rho2sigma_8;
use part15::mgga_c_tpssloc_kxc_pol_part15_v3rho2lapl_v3rho2tau;
use part16::mgga_c_tpssloc_kxc_pol_part16_v3rhosigma2_v3rhosigmalapl;
use part17::mgga_c_tpssloc_kxc_pol_part17_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2;
use part18::mgga_c_tpssloc_kxc_pol_part18_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_tpssloc_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part2_v3rho3_0(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part3_v3rho3_1(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part4_v3rho3_2(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part5_v3rho3_3(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part6_v3rho2sigma_0(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part7_v3rho2sigma_1(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part8_v3rho2sigma_2(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part9_v3rho2sigma_3(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part10_v3rho2sigma_4(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part11_v3rho2sigma_5(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part12_v3rho2sigma_6(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part13_v3rho2sigma_7(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part14_v3rho2sigma_8(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part15_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2lapl, v3rho2tau, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part16_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part17_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, dens_threshold, zeta_threshold);
    mgga_c_tpssloc_kxc_pol_part18_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc(rho, sigma, lapl, tau, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, dens_threshold, zeta_threshold);
}
