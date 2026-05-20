//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 578/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk578<F: Float>(t1995: F, t241: F, t67: F, t1376: F, t566: F, t68: F, t3787: F, t562: F, t193: F, t532: F) -> (F, F, F, F) {
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3886 = F::new(1.0) / t1376 / t566;
    let t3887 = t68 * t3886;
    let t3897 = t3787 * t562;
    let t3918 = t193 * t532;
    (t3870, t3887, t3897, t3918)
}
