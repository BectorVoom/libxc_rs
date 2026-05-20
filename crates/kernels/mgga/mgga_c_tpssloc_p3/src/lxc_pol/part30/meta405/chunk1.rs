//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1542/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1542<F: Float>(t1023: F, t5677: F, t10408: F, t1036: F, t5905: F, t1041: F, t10876: F, t10883: F, t10952: F, t13995: F, t14158: F, t14160: F, t17972: F, t17976: F, t17980: F, t17984: F, t17988: F, t17991: F, t17994: F, t3070: F, t3109: F, t4579: F, t5869: F, t5880: F, t973: F) -> (F, F) {
    let t17997 = t5677 * t1023;
    let t17998 = t10408 * t17997;
    let t18005 = t5905 * t1036;
    let t18007 = -t10952 * t5880 / F::new(3072.0) + t1041 * t17972 / F::new(768.0) - t1041 * t17976 / F::new(1152.0) + t10883 * t17980 / F::new(3072.0) - t10876 * t17984 / F::new(512.0) - t14158 - t14160 / F::new(648.0) + t973 * t17988 / F::new(48.0) - t973 * t17991 / F::new(72.0) - t973 * t17994 / F::new(36.0) + F::new(5.0) / F::new(13824.0) * t3070 * t17998 + t13995 * t4579 / F::new(2304.0) - t3109 * t5869 / F::new(576.0) + t18005 / F::new(4608.0);
    (t17998, t18007)
}
