//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1067/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1067<F: Float>(t116: F, t244: F, t207: F, t40419: F, t9538: F, t154: F, t1891: F, t205: F, t792: F, t9558: F, t40394: F, t40399: F, t786: F, t9580: F, t2566: F, t2570: F) -> (F, F, F, F, F, F, F) {
    let t41146 = t244 * t116;
    let t41155 = 0.26851851851851851851e-2 * t40419 * t207 * t9538;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    let t41185 = 0.69444444444444444445e-4 * t40394 * t207 * t40399;
    let t41189 = t9580 * t786;
    let t41196 = t2566 * t2570;
    (t41146, t41155, t41161, t41170, t41185, t41189, t41196)
}
