//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2321/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2321<F: Float>(t15492: F, t7339: F, t15734: F, t7345: F, t25588: F, t461: F, t7324: F, t1244: F, t1742: F, t3068: F, t1210: F, t24721: F, t27691: F, sigma2: F) -> (F, F, F, F, F) {
    let t95545 = t7339 * t15492 / F::cast_from(1152.0_f64);
    let t95550 = t7345 * t15734;
    let t95556 = t7324 * t25588 * t461;
    let t95566 = t1244 * sigma2 * t1742 * t3068;
    let t95571 = F::cast_from(0.20186378047070195428e-3_f64) * t24721 * t1210 * t27691;
    (t95545, t95550, t95556, t95566, t95571)
}
