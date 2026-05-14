//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1131/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1131<F: Float>(t116: F, t244: F, t212: F, t2379: F, t2586: F, t2563: F, t9529: F, t207: F, t40419: F, t9538: F, t41083: F, t789: F, t41011: F, t9561: F, t154: F, t1891: F) -> (F, F, F, F, F, F) {
    let t41146 = t244 * t116;
    let t41149 = t2586 * t41146 * t212 * t2379;
    let t41151 = t2563 * t9529;
    let t41155 = 0.26851851851851851851e-2 * t40419 * t207 * t9538;
    let t41156 = t41083 * t789;
    let t41158 = t41011 * t9561;
    let t41160 = t154 * t1891;
    (t41149, t41151, t41155, t41156, t41158, t41160)
}
