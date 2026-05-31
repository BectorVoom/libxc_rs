//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 900/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk900<F: Float>(t2039: F, t33211: F, t7467: F, t88: F, t7801: F, t8601: F, t31758: F, t7687: F, t1983: F, t5161: F, t8640: F, t7688: F, t8607: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33595 = F::cast_from(2.0_f64) * t33211 * t2039;
    let t33596 = t88 * t7467;
    let t33598 = F::cast_from(2.0_f64) * t33596 * t2039;
    let t33600 = F::cast_from(2.0_f64) * t8601 * t7801;
    let t33603 = t31758 * t7687;
    let t33605 = F::cast_from(3.0_f64) * t1983 * t33603;
    let t33610 = t8640 * t5161;
    let t33611 = t1983 * t33610;
    let t33615 = F::cast_from(3.0_f64) * t8607 * t7688;
    (t33595, t33596, t33598, t33600, t33603, t33605, t33610, t33611, t33615)
}
