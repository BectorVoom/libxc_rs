//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1109/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1109<F: Float>(t32984: F, t33012: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t2522: F, t30757: F, t30770: F, t32885: F, t6670: F, t7540: F, t8366: F, t8370: F, t870: F) -> (F, F) {
    let t33013 = t32984 + t33012;
    let t33043 = t193 * t202 * t32885 * t870 + F::cast_from(3.0_f64) * t1484 * t2522 * t8366 - F::cast_from(3.0_f64) * t1484 * t2522 * t8370 - t1530 * t1877 * t30757 + F::cast_from(2.0_f64) * t1530 * t1877 * t30770 - F::cast_from(2.0_f64) * t1877 * t6670 * t7540;
    (t33013, t33043)
}
