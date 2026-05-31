//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1021/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1021<F: Float>(t1458: F, t23877: F, t23880: F, t26509: F, t26523: F, t26533: F, t26535: F, t26537: F, t26539: F, t26541: F, t26544: F, t26547: F, t26549: F, t26552: F, t26554: F, t4072: F, t5376: F, t577: F, t671: F, t7010: F) -> F {
    let t26555 = F::cast_from(0.45e1_f64) * t26509 * t577 + F::cast_from(0.135e2_f64) * t26523 * t671 + F::cast_from(0.135e2_f64) * t23877 * t1458 + F::cast_from(27.0_f64) * t23880 * t5376 + F::cast_from(0.135e2_f64) * t7010 * t4072 + t26533 + t26535 + t26537 + t26539 + t26541 + t26544 + t26547 + t26549 + t26552 + t26554;
    t26555
}
