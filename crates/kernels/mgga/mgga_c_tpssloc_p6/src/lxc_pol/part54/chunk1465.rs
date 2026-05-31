//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1465/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1465<F: Float>(t120944: F, t120947: F, t120948: F, t120954: F, t120958: F, t120962: F, t120964: F, t120966: F, t120968: F, t1849: F, t2039: F, t2040: F, t26878: F, t27858: F, t32359: F, t652: F, t8690: F, t96238: F) -> F {
    let t124900 = -F::cast_from(2.0_f64) * t2039 * t27858 * t652 + t1849 * t32359 - F::cast_from(2.0_f64) * t2040 * t96238 - t26878 * t8690 + t120944 + t120947 + t120948 - t120954 + t120958 - t120962 - t120964 - t120966 - t120968;
    t124900
}
