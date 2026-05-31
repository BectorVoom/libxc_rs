//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 957/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk957<F: Float>(t1098: F, t12535: F, t1554: F, t3025: F, t219: F, t4294: F, t1270: F, t4519: F, t2222: F, t4435: F, t4377: F, t72: F) -> (F, F, F, F, F, F) {
    let t12537 = t1098 * t12535 / F::cast_from(432.0_f64);
    let t12550 = t1554 * t3025;
    let t12557 = t4294 * t219;
    let t12673 = t4519 * t1270;
    let t12677 = t4435 * t2222;
    let t12686 = t4377 * t72;
    (t12537, t12550, t12557, t12673, t12677, t12686)
}
