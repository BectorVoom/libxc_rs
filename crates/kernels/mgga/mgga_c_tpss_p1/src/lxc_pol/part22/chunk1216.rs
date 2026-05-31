//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1216/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1216<F: Float>(t17944: F, t17971: F, t17947: F, t17950: F, t17952: F, t17957: F, t17961: F, t17965: F, t17967: F, t17969: F, t17975: F, t17977: F, t17979: F) -> (F, F, F) {
    let t18737 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t17944;
    let t18746 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t17971;
    let t18750 = t18737 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t17947 + t17950 / F::cast_from(8.0_f64) - t17952 / F::cast_from(24.0_f64) + t17957 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t17961 + t17965 / F::cast_from(96.0_f64) - t17967 / F::cast_from(768.0_f64) - t17969 / F::cast_from(768.0_f64) + t18746 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t17975 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t17977 - t17979 / F::cast_from(192.0_f64);
    (t18737, t18746, t18750)
}
