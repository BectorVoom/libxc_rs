//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1617/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617<F: Float>(t18316: F, t18337: F, t18390: F, t18951: F, t18989: F, t19029: F, t19075: F, t19117: F, t466: F, t5068: F, t6260: F, t18940: F, t491: F) -> (F, F, F, F) {
    let t19120 = t18316 + t18337 + t18390 + t18951 + t18989 + t19029 + t19075 + t19117;
    let t19121 = t466 * t19120;
    let t19123 = t6260 * t5068;
    let t19128 = t491 * t18940;
    (t19120, t19121, t19123, t19128)
}
