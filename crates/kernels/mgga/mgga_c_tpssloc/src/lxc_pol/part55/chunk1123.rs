//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1123/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1123<F: Float>(t34295: F, t462: F, t1734: F, t8882: F, t1246: F, t34277: F, t493: F, t1244: F, t1729: F, t2121: F, t32456: F, t34285: F, t34288: F, t34292: F, t470: F, t7283: F, t7373: F, t7999: F, t8892: F, t8895: F) -> (F, F, F, F) {
    let t34296 = t462 * t34295;
    let t34300 = t8882 * t1734;
    let t34301 = t34300 * t1246;
    let t34303 = t493 * t34277;
    let t34305 = -F::cast_from(0.43864908449286038307e-1_f64) * t7999 * t8892 + t32456 - F::cast_from(0.54831135561607547883e-2_f64) * t7283 * t34285 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t34288 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t34292 + F::cast_from(0.16449340668482264365e-1_f64) * t2121 * t34296 + t1729 * t8895 + t1244 * t34301 + t470 * t34303;
    (t34300, t34301, t34303, t34305)
}
