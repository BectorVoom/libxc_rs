//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 799/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk799<F: Float>(t33334: F, t533: F, t1390: F, t1983: F, t7802: F, t8526: F, t1799: F, t2018: F, t24432: F, t22574: F, t7685: F, t8644: F, t191: F, t192: F, t7900: F, t2020: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33335 = t533 * t33334;
    let t33336 = t33335 * t1390;
    let t33337 = t1983 * t33336;
    let t33345 = 2.0 * t8526 * t7802;
    let t33357 = t2018 * t1799;
    let t33358 = t24432 * t33357;
    let t33360 = 3.0 * t22574 * t33358;
    let t33361 = t7685 * t8644;
    let t33363 = t7900 * t191 * t192;
    let t33364 = t33363 * t2020;
    (t33335, t33336, t33337, t33345, t33357, t33358, t33360, t33361, t33363, t33364)
}
