//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1106/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1106<F: Float>(t25: F, t7540: F, t1408: F, t1877: F, t2522: F, t30757: F, t30770: F, t32886: F, t6670: F, t7475: F, t7545: F, t8366: F, t8370: F) -> (F, F) {
    let t32899 = t25 * t7540;
    let t32907 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8366 * t7475 + t1877 * t32886 * t25 / F::cast_from(2.0_f64) - t1877 * t30757 * t7545 / F::cast_from(2.0_f64) + t1877 * t8366 * t1408 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8370 * t7475 - t1877 * t6670 * t32899 + t1877 * t30770 * t7545 - t1877 * t8370 * t1408 / F::cast_from(2.0_f64);
    (t32899, t32907)
}
