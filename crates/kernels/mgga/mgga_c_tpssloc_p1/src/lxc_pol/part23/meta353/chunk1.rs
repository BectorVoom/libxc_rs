//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1149/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149<F: Float>(t238: F, t244: F, t248: F, t40445: F, t116: F, t207: F, t40419: F, t9538: F, t154: F, t1891: F, t205: F, t792: F, t9558: F) -> (F, F, F, F, F) {
    let t41139 = F::new(13685.0) / F::new(31104.0) * t238 * t40445 * t244 * t248;
    let t41146 = t244 * t116;
    let t41155 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t207 * t9538;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    (t41139, t41146, t41155, t41161, t41170)
}
