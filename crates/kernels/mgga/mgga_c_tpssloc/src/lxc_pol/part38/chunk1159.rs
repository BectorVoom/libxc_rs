//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1159/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1159<F: Float>(t12832: F, t16505: F, t3: F, t112: F, t5363: F, t111: F, t1851: F, t2319: F, t576: F, t4072: F, t671: F, t1458: F, t2363: F, t12521: F, t12524: F, t12813: F, t1401: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F) -> (F, F, F, F, F, F, F, F) {
    let t16506 = t12832 + t16505;
    let t16507 = t3 * t16506;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16535 = t576 * t2319;
    let t16538 = t4072 * t671;
    let t16541 = t1458 * t2363;
    let t16546 = 0.45e1 * t16506 * t577 + 27.0 * t16521 * t671 + 27.0 * t16524 * t2319 + 0.135e2 * t5371 * t2363 + 0.135e2 * t12521 * t1458 + 54.0 * t12524 * t5376 + 27.0 * t3938 * t4072 + 27.0 * t16535 * t1458 + 54.0 * t3941 * t16538 + 27.0 * t3941 * t16541 + 0.135e2 * t1401 * t12813;
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546)
}
