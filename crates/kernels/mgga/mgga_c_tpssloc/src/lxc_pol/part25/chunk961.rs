//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 961/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk961<F: Float>(t12407: F, t3805: F, t3806: F, t12402: F, t1352: F, t5248: F, t1995: F, t67: F, t246: F, t3734: F, t550: F, t12368: F, t3807: F) -> (F, F, F, F) {
    let t12409 = t3805 * t3806 * t12407;
    let t12413 = t5248 * t12402 * t1352;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12420 = t550 * t3734;
    let t12422 = t12419 * t3806 * t12420;
    let t12426 = t3805 * t12368 * t3807;
    (t12409, t12413, t12422, t12426)
}
