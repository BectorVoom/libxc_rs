//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2048/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048<F: Float>(t2229: F, t59: F, t60: F, t535: F, t9538: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t12248: F, t1372: F) -> (F, F, F, F, F) {
    let t40419 = t59 / t60 / t2229;
    let t40422 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t535 * t9538;
    let t40445 = t6597 * t241;
    let t40449 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t555 * t40445 * t557 * t248;
    let t40492 = t12248 * t1372;
    (t40419, t40422, t40445, t40449, t40492)
}
