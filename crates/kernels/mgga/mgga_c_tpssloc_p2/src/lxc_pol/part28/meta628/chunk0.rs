//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1967/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967<F: Float>(t1408: F, t1877: F, t2057: F, t23302: F, t24191: F, t24335: F, t25021: F, t25028: F, t2522: F, t26563: F, t26740: F, t26744: F, t26756: F, t47645: F, t606: F, t7110: F, t7545: F, t7809: F, t84791: F, t84797: F, t86707: F, t86714: F, t86727: F, t86771: F, t87953: F, t87978: F, t87988: F) -> F {
    let t92270 = t1877 * t24335 * t1408 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t26563 * t87978 + t26756 * t86714 - F::cast_from(3.0_f64) * t84797 * t25021 + F::cast_from(2.0_f64) * t26756 * t86771 - F::cast_from(3.0_f64) * t24191 * t86727 - t1877 * t84791 * t7545 / F::cast_from(2.0_f64) + t1877 * t26740 * t606 - t1877 * t26744 * t23302 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t87953 + F::cast_from(3.0_f64) * t2522 * t7110 * t25028 + F::cast_from(3.0_f64) * t47645 * t7809 + F::cast_from(3.0_f64) * t24191 * t87988 - F::cast_from(3.0_f64) * t26563 * t86707;
    t92270
}
