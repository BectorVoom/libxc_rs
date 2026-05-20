//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk831;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk832;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk833;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta170<F: Float>(t25: F, t28: F, t515: F, t1298: F, t2249: F, t3665: F, t518: F, t1302: F, t3231: F, t3673: F, zeta_threshold: F, t215: F, t2559: F, t535: F, t1314: F, t782: F, t1317: F, t2566: F, t795: F, t154: F, t557: F, t205: F, t1307: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3704, t3711, t3719) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk831::<F>(t25, t28, t515, t1298, t2249, t3665, t518, t1302, t3231, t3673, zeta_threshold);
        let (t3725, t3726) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk832::<F>(t215, t2559, t535, t1314, t782);
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk833::<F>(t1317, t3726, t2566, t535, t795, t154, t557);
        let (t3733, t3734) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk834::<F>(t205, t3732, t1307);
    (t3704, t3711, t3719, t3725, t3726, t3727, t3731, t3732, t3733, t3734)
}
