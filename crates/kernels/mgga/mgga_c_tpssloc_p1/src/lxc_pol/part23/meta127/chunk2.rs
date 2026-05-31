//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 630/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk630<F: Float>(t31: F, t5398: F, t65: F, t1410: F, t1426: F, t2267: F, t5392: F, t43: F, t48: F, t480: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5399 = t31 * t5398;
    let t5400 = t5399 * t65;
    let t5403 = t1410 * t1426;
    let t5408 = t2267 * t5392;
    let t5411 = t43 * t5398;
    let t5415 = F::cast_from(1.0_f64) / t48 / t480;
    let t5416 = sigma2 * t5415;
    (t5399, t5400, t5403, t5408, t5411, t5415, t5416)
}
