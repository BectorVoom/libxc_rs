//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1462/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1462<F: Float>(t104990: F, t106921: F, t106923: F, t106932: F, t106934: F, t106937: F, t106939: F, t106941: F, t106953: F, t106958: F, t106960: F, t108902: F, t109029: F, t1458: F, t20347: F, t27863: F, t33690: F, t5493: F, t7266: F) -> F {
    let t109976 = F::new(6.0) * t104990 * t1458 + F::new(2.0) * t20347 * t7266 + F::new(6.0) * t27863 * t5493 + F::new(6.0) * t33690 * t5493 + t106921 + t106923 + t106932 + t106934 + t106937 + t106939 + t106941 + t106953 + t106958 + t106960 + F::new(6.0) * t108902 + t109029;
    t109976
}
