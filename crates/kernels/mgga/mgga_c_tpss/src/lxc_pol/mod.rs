//! MGGA_C_TPSS lxc pol kernel — lxc_pol (nested-by-output, 93 parts).
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
mod part19;
mod part20;
mod part21;
mod part22;
mod part23;
mod part24;
mod part25;
mod part26;
mod part27;
mod part28;
mod part29;
mod part30;
mod part31;
mod part32;
mod part33;
mod part34;
mod part35;
mod part36;
mod part37;
mod part38;
mod part39;
mod part40;
mod part41;
mod part42;
mod part43;
mod part44;
mod part45;
mod part46;
mod part47;
mod part48;
mod part49;
mod part50;
mod part51;
mod part52;
mod part53;
mod part54;
mod part55;
mod part56;
mod part57;
mod part58;
mod part59;
mod part60;
mod part61;
mod part62;
mod part63;
mod part64;
mod part65;
mod part66;
mod part67;
mod part68;
mod part69;
mod part70;
mod part71;
mod part72;
mod part73;
mod part74;
mod part75;
mod part76;
mod part77;
mod part78;
mod part79;
mod part80;
mod part81;
mod part82;
mod part83;
mod part84;
mod part85;
mod part86;
mod part87;
mod part88;
mod part89;
mod part90;
mod part91;
mod part92;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_tpss_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_tpss_lxc_pol_part1_v3rho3_0;
use part2::mgga_c_tpss_lxc_pol_part2_v3rho3_1;
use part3::mgga_c_tpss_lxc_pol_part3_v3rho3_2;
use part4::mgga_c_tpss_lxc_pol_part4_v3rho3_3;
use part5::mgga_c_tpss_lxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part6::mgga_c_tpss_lxc_pol_part6_v3rho2sigma_3;
use part7::mgga_c_tpss_lxc_pol_part7_v3rho2sigma_4;
use part8::mgga_c_tpss_lxc_pol_part8_v3rho2sigma_5;
use part9::mgga_c_tpss_lxc_pol_part9_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8_v3rho2lapl_0_v3rho_etc;
use part10::mgga_c_tpss_lxc_pol_part10_v3rho2tau;
use part11::mgga_c_tpss_lxc_pol_part11_v3rhosigma2_v3rhosigmalapl;
use part12::mgga_c_tpss_lxc_pol_part12_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc;
use part13::mgga_c_tpss_lxc_pol_part13_v4rho4_0;
use part14::mgga_c_tpss_lxc_pol_part14_v4rho4_1;
use part15::mgga_c_tpss_lxc_pol_part15_v4rho4_2;
use part16::mgga_c_tpss_lxc_pol_part16_v4rho4_3;
use part17::mgga_c_tpss_lxc_pol_part17_v4rho4_4;
use part18::mgga_c_tpss_lxc_pol_part18_v4rho3sigma_0;
use part19::mgga_c_tpss_lxc_pol_part19_v4rho3sigma_1;
use part20::mgga_c_tpss_lxc_pol_part20_v4rho3sigma_2;
use part21::mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3;
use part22::mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4;
use part23::mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5;
use part24::mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6;
use part25::mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7;
use part26::mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8;
use part27::mgga_c_tpss_lxc_pol_part27_v4rho3sigma_9;
use part28::mgga_c_tpss_lxc_pol_part28_v4rho3sigma_10;
use part29::mgga_c_tpss_lxc_pol_part29_v4rho3sigma_11_v4rho3lapl;
use part30::mgga_c_tpss_lxc_pol_part30_v4rho3tau_0;
use part31::mgga_c_tpss_lxc_pol_part31_v4rho3tau_1;
use part32::mgga_c_tpss_lxc_pol_part32_v4rho3tau_2;
use part33::mgga_c_tpss_lxc_pol_part33_v4rho3tau_3;
use part34::mgga_c_tpss_lxc_pol_part34_v4rho3tau_4;
use part35::mgga_c_tpss_lxc_pol_part35_v4rho3tau_5;
use part36::mgga_c_tpss_lxc_pol_part36_v4rho3tau_6;
use part37::mgga_c_tpss_lxc_pol_part37_v4rho3tau_7;
use part38::mgga_c_tpss_lxc_pol_part38_v4rho2sigma2_0;
use part39::mgga_c_tpss_lxc_pol_part39_v4rho2sigma2_1;
use part40::mgga_c_tpss_lxc_pol_part40_v4rho2sigma2_2;
use part41::mgga_c_tpss_lxc_pol_part41_v4rho2sigma2_3;
use part42::mgga_c_tpss_lxc_pol_part42_v4rho2sigma2_4;
use part43::mgga_c_tpss_lxc_pol_part43_v4rho2sigma2_5;
use part44::mgga_c_tpss_lxc_pol_part44_v4rho2sigma2_6;
use part45::mgga_c_tpss_lxc_pol_part45_v4rho2sigma2_7;
use part46::mgga_c_tpss_lxc_pol_part46_v4rho2sigma2_8;
use part47::mgga_c_tpss_lxc_pol_part47_v4rho2sigma2_9;
use part48::mgga_c_tpss_lxc_pol_part48_v4rho2sigma2_10;
use part49::mgga_c_tpss_lxc_pol_part49_v4rho2sigma2_11;
use part50::mgga_c_tpss_lxc_pol_part50_v4rho2sigma2_12;
use part51::mgga_c_tpss_lxc_pol_part51_v4rho2sigma2_13;
use part52::mgga_c_tpss_lxc_pol_part52_v4rho2sigma2_14;
use part53::mgga_c_tpss_lxc_pol_part53_v4rho2sigma2_15;
use part54::mgga_c_tpss_lxc_pol_part54_v4rho2sigma2_16;
use part55::mgga_c_tpss_lxc_pol_part55_v4rho2sigma2_17_v4rho2sigmalapl;
use part56::mgga_c_tpss_lxc_pol_part56_v4rho2sigmatau_0;
use part57::mgga_c_tpss_lxc_pol_part57_v4rho2sigmatau_1;
use part58::mgga_c_tpss_lxc_pol_part58_v4rho2sigmatau_2_v4rho2sigmatau_3;
use part59::mgga_c_tpss_lxc_pol_part59_v4rho2sigmatau_4;
use part60::mgga_c_tpss_lxc_pol_part60_v4rho2sigmatau_5;
use part61::mgga_c_tpss_lxc_pol_part61_v4rho2sigmatau_6;
use part62::mgga_c_tpss_lxc_pol_part62_v4rho2sigmatau_7;
use part63::mgga_c_tpss_lxc_pol_part63_v4rho2sigmatau_8;
use part64::mgga_c_tpss_lxc_pol_part64_v4rho2sigmatau_9;
use part65::mgga_c_tpss_lxc_pol_part65_v4rho2sigmatau_10;
use part66::mgga_c_tpss_lxc_pol_part66_v4rho2sigmatau_11;
use part67::mgga_c_tpss_lxc_pol_part67_v4rho2sigmatau_12;
use part68::mgga_c_tpss_lxc_pol_part68_v4rho2sigmatau_13;
use part69::mgga_c_tpss_lxc_pol_part69_v4rho2sigmatau_14_v4rho2sigmatau_15;
use part70::mgga_c_tpss_lxc_pol_part70_v4rho2sigmatau_16;
use part71::mgga_c_tpss_lxc_pol_part71_v4rho2sigmatau_17;
use part72::mgga_c_tpss_lxc_pol_part72_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
use part73::mgga_c_tpss_lxc_pol_part73_v4rhosigma3_0_v4rhosigma3_1;
use part74::mgga_c_tpss_lxc_pol_part74_v4rhosigma3_2_v4rhosigma3_3;
use part75::mgga_c_tpss_lxc_pol_part75_v4rhosigma3_4_v4rhosigma3_5_v4rhosigma3_6;
use part76::mgga_c_tpss_lxc_pol_part76_v4rhosigma3_7_v4rhosigma3_8_v4rhosigma3_9;
use part77::mgga_c_tpss_lxc_pol_part77_v4rhosigma3_10_v4rhosigma3_11;
use part78::mgga_c_tpss_lxc_pol_part78_v4rhosigma3_12_v4rhosigma3_13;
use part79::mgga_c_tpss_lxc_pol_part79_v4rhosigma3_14;
use part80::mgga_c_tpss_lxc_pol_part80_v4rhosigma3_15_v4rhosigma3_16;
use part81::mgga_c_tpss_lxc_pol_part81_v4rhosigma3_17_v4rhosigma3_18;
use part82::mgga_c_tpss_lxc_pol_part82_v4rhosigma3_19_v4rhosigma2lapl_v4rhosigma2tau_0;
use part83::mgga_c_tpss_lxc_pol_part83_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigma2tau_3;
use part84::mgga_c_tpss_lxc_pol_part84_v4rhosigma2tau_4_v4rhosigma2tau_5_v4rhosigma2tau_6_v4rhosigm_etc;
use part85::mgga_c_tpss_lxc_pol_part85_v4rhosigma2tau_9_v4rhosigma2tau_10_v4rhosigma2tau_11_v4rhosi_etc;
use part86::mgga_c_tpss_lxc_pol_part86_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhosigma2tau_15;
use part87::mgga_c_tpss_lxc_pol_part87_v4rhosigma2tau_16_v4rhosigma2tau_17_v4rhosigma2tau_18_v4rhos_etc;
use part88::mgga_c_tpss_lxc_pol_part88_v4rhosigma2tau_20_v4rhosigma2tau_21_v4rhosigma2tau_22_v4rhos_etc;
use part89::mgga_c_tpss_lxc_pol_part89_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3__etc;
use part90::mgga_c_tpss_lxc_pol_part90_v4rhotau3_v4sigma4_v4sigma3lapl;
use part91::mgga_c_tpss_lxc_pol_part91_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau;
use part92::mgga_c_tpss_lxc_pol_part92_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_tpss_lxc_pol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_beta: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_tpss_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part1_v3rho3_0(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part2_v3rho3_1(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part3_v3rho3_2(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part4_v3rho3_3(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part6_v3rho2sigma_3(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part7_v3rho2sigma_4(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part8_v3rho2sigma_5(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part9_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8_v3rho2lapl_0_v3rho_etc(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part10_v3rho2tau(rho, sigma, lapl, tau, v3rho2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part11_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part12_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part13_v4rho4_0(rho, sigma, lapl, tau, v4rho4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part14_v4rho4_1(rho, sigma, lapl, tau, v4rho4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part15_v4rho4_2(rho, sigma, lapl, tau, v4rho4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part16_v4rho4_3(rho, sigma, lapl, tau, v4rho4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part17_v4rho4_4(rho, sigma, lapl, tau, v4rho4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part18_v4rho3sigma_0(rho, sigma, lapl, tau, v4rho3sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part19_v4rho3sigma_1(rho, sigma, lapl, tau, v4rho3sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part20_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part27_v4rho3sigma_9(rho, sigma, lapl, tau, v4rho3sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part28_v4rho3sigma_10(rho, sigma, lapl, tau, v4rho3sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part29_v4rho3sigma_11_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part30_v4rho3tau_0(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part31_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part32_v4rho3tau_2(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part33_v4rho3tau_3(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part34_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part35_v4rho3tau_5(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part36_v4rho3tau_6(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part37_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part38_v4rho2sigma2_0(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part39_v4rho2sigma2_1(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part40_v4rho2sigma2_2(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part41_v4rho2sigma2_3(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part42_v4rho2sigma2_4(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part43_v4rho2sigma2_5(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part44_v4rho2sigma2_6(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part45_v4rho2sigma2_7(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part46_v4rho2sigma2_8(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part47_v4rho2sigma2_9(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part48_v4rho2sigma2_10(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part49_v4rho2sigma2_11(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part50_v4rho2sigma2_12(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part51_v4rho2sigma2_13(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part52_v4rho2sigma2_14(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part53_v4rho2sigma2_15(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part54_v4rho2sigma2_16(rho, sigma, lapl, tau, v4rho2sigma2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part55_v4rho2sigma2_17_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part56_v4rho2sigmatau_0(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part57_v4rho2sigmatau_1(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part58_v4rho2sigmatau_2_v4rho2sigmatau_3(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part59_v4rho2sigmatau_4(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part60_v4rho2sigmatau_5(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part61_v4rho2sigmatau_6(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part62_v4rho2sigmatau_7(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part63_v4rho2sigmatau_8(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part64_v4rho2sigmatau_9(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part65_v4rho2sigmatau_10(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part66_v4rho2sigmatau_11(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part67_v4rho2sigmatau_12(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part68_v4rho2sigmatau_13(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part69_v4rho2sigmatau_14_v4rho2sigmatau_15(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part70_v4rho2sigmatau_16(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part71_v4rho2sigmatau_17(rho, sigma, lapl, tau, v4rho2sigmatau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part72_v4rho2lapl2_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part73_v4rhosigma3_0_v4rhosigma3_1(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part74_v4rhosigma3_2_v4rhosigma3_3(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part75_v4rhosigma3_4_v4rhosigma3_5_v4rhosigma3_6(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part76_v4rhosigma3_7_v4rhosigma3_8_v4rhosigma3_9(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part77_v4rhosigma3_10_v4rhosigma3_11(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part78_v4rhosigma3_12_v4rhosigma3_13(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part79_v4rhosigma3_14(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part80_v4rhosigma3_15_v4rhosigma3_16(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part81_v4rhosigma3_17_v4rhosigma3_18(rho, sigma, lapl, tau, v4rhosigma3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part82_v4rhosigma3_19_v4rhosigma2lapl_v4rhosigma2tau_0(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part83_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigma2tau_3(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part84_v4rhosigma2tau_4_v4rhosigma2tau_5_v4rhosigma2tau_6_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part85_v4rhosigma2tau_9_v4rhosigma2tau_10_v4rhosigma2tau_11_v4rhosi_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part86_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhosigma2tau_15(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part87_v4rhosigma2tau_16_v4rhosigma2tau_17_v4rhosigma2tau_18_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part88_v4rhosigma2tau_20_v4rhosigma2tau_21_v4rhosigma2tau_22_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part89_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3__etc(rho, sigma, lapl, tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part90_v4rhotau3_v4sigma4_v4sigma3lapl(rho, sigma, lapl, tau, v4rhotau3, v4sigma4, v4sigma3lapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part91_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau(rho, sigma, lapl, tau, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_lxc_pol_part92_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4_etc(rho, sigma, lapl, tau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
}
