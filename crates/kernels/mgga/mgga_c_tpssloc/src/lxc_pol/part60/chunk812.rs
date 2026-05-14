//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 812/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk812<F: Float>(t33518: F, t33552: F, t113: F, t7756: F, t8607: F, t1442: F, t8595: F, t1873: F, t27188: F, t33234: F, t7042: F, t7467: F, t2039: F, t33211: F, t88: F, t7801: F, t8601: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33553 = t33518 + t33552;
    let t33554 = t113 * t33553;
    let t33555 = t8607 * t7756;
    let t33556 = t1442 * t8595;
    let t33583 = 2.0 * t27188 * t1873;
    let t33585 = 2.0 * t33234 * t1873;
    let t33587 = 2.0 * t7042 * t7467;
    let t33595 = 2.0 * t33211 * t2039;
    let t33596 = t88 * t7467;
    let t33598 = 2.0 * t33596 * t2039;
    let t33600 = 2.0 * t8601 * t7801;
    (t33553, t33554, t33555, t33556, t33583, t33585, t33587, t33595, t33596, t33598, t33600)
}
