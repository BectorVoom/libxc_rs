//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2265/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2265<F: Float>(t1437: F, t6509: F, t1863: F, t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t83722: F, t83741: F, t83778: F, t90072: F, t90076: F, t90080: F, t90087: F) -> F {
    let t90090 = t6509 * t1437;
    let t90091 = t1863 * t90090;
    let t90094 = t1864 * t4021;
    let t90095 = t1863 * t90094;
    let t90098 = t9231 * t1410;
    let t90101 = t2240 * t3961;
    let t90104 = t2240 * t3967;
    let t90107 = -F::cast_from(10.0_f64) * t83741 * t26009 - F::cast_from(10.0_f64) * t22544 * t90072 - F::cast_from(10.0_f64) * t22544 * t90076 - F::cast_from(5.0_f64) * t22544 * t90080 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t83722 * t26013 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t83778 * t26013 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t90087 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t90091 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t90095 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90098 * t22551 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90101 * t22551 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90104 * t22551;
    t90107
}
