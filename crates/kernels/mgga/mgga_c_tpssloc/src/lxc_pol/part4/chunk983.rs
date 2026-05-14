//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 983/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk983<F: Float>(t1041: F, t10876: F, t10883: F, t10952: F, t13995: F, t14158: F, t14160: F, t17972: F, t17976: F, t17980: F, t17984: F, t17988: F, t17991: F, t17994: F, t17998: F, t18005: F, t3070: F, t3109: F, t4579: F, t5869: F, t5880: F, t973: F) -> (F,) {
    let t18007 = -t10952 * t5880 / 3072.0 + t1041 * t17972 / 768.0 - t1041 * t17976 / 1152.0 + t10883 * t17980 / 3072.0 - t10876 * t17984 / 512.0 - t14158 - t14160 / 648.0 + t973 * t17988 / 48.0 - t973 * t17991 / 72.0 - t973 * t17994 / 36.0 + 5.0 / 13824.0 * t3070 * t17998 + t13995 * t4579 / 2304.0 - t3109 * t5869 / 576.0 + t18005 / 4608.0;
    (t18007,)
}
