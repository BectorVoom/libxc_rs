//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1292/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1292<F: Float>(t114449: F, t114451: F, t120774: F, t120780: F, t120783: F, t120814: F, t120852: F, t1396: F, t1398: F, t1852: F, t2023: F, t26555: F, t31288: F, t33196: F, t5364: F, t7003: F, t7020: F, t7759: F, t7774: F, t8509: F) -> F {
    let t120855 = F::cast_from(2.0_f64) * t7759 * t7020 + F::cast_from(2.0_f64) * t120774 + F::cast_from(2.0_f64) * t7003 * t7774 + F::cast_from(2.0_f64) * t2023 * t26555 + t114449 + t114451 + t120780 + t5364 * t8509 + t1852 * t31288 + t120783 + t1396 * t33196 + t1398 * (t120814 + t120852);
    t120855
}
