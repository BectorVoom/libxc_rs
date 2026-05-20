//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3021/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3021<F: Float>(t15051: F, t51667: F, t4857: F, t4781: F, t1118: F, t3264: F, t18238: F, t690: F) -> (F, F, F, F, F) {
    let t63280 = F::new(24.0) * t51667 * t15051;
    let t63283 = t4857 * t4857;
    let t63287 = t4781 * t4781;
    let t63290 = F::new(4.0) * t3264 * t63287 * t1118;
    let t63291 = t690 * t18238;
    (t63280, t63283, t63287, t63290, t63291)
}
