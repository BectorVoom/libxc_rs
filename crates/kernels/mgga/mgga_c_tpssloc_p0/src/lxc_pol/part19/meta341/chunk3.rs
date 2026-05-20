//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1218/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1218<F: Float>(t212: F, t2553: F, t2586: F, t9523: F, t9525: F, t9577: F, t116: F, t244: F, t2379: F, t2563: F, t9529: F, t207: F, t40419: F, t9538: F) -> (F, F, F, F, F) {
    let t41142 = t2586 * t9523 * t212 * t2553;
    let t41144 = t9577 * t9525;
    let t41146 = t244 * t116;
    let t41149 = t2586 * t41146 * t212 * t2379;
    let t41151 = t2563 * t9529;
    let t41155 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t207 * t9538;
    (t41142, t41144, t41149, t41151, t41155)
}
