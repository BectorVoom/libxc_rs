//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2238/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238<F: Float>(t12939: F, t13126: F, t2244: F, t2745: F, t868: F, t16693: F, t9682: F, t1409: F, t707: F, t9862: F, t13123: F, t9467: F) -> (F, F, F, F, F) {
    let t46361 = F::cast_from(72.0_f64) * t12939 * t13126 * t2244;
    let t46362 = t2745 * t868;
    let t46367 = F::cast_from(36.0_f64) * t16693 * t9682;
    let t46369 = t707 * t9862 * t1409;
    let t46370 = F::cast_from(4.0_f64) * t46369;
    let t46371 = t13123 * t9467;
    (t46361, t46362, t46367, t46370, t46371)
}
