//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1016;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1017;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1018;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1019;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1020;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1021;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1022;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta156<F: Float>(t570: F, t515: F, t25: F, t1298: F, t2249: F, t3665: F, t518: F, zeta_threshold: F, t28: F, t1302: F, t3231: F, t3673: F, t215: F, t2559: F, t535: F, t1314: F, t782: F, t1317: F, t2566: F, t795: F, t154: F, t557: F, t205: F, t1307: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3700, t3701) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1016::<F>(t570);
        let t3704 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1017::<F>(t515);
        let (t3710, t3711) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1018::<F>(t25, t1298, t2249, t3665, t3704, t518, zeta_threshold);
        let t3719 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1019::<F>(t28, t1302, t3231, t3673, t3711, t3710, zeta_threshold);
        let (t3725, t3726) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1020::<F>(t215, t2559, t535, t1314, t782);
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1021::<F>(t1317, t3726, t2566, t535, t795, t154, t557);
        let t3733 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1022::<F>(t205, t3732);
        let t3734 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1023::<F>(t1307);
    (t3700, t3701, t3704, t3711, t3719, t3725, t3726, t3727, t3731, t3732, t3733, t3734)
}
