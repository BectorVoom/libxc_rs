//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1235/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1235<F: Float>(t33518: F, t33552: F, t113: F, t7756: F, t8607: F, t1442: F, t8595: F, t1873: F, t27188: F, t33234: F, t7042: F, t7467: F) -> (F, F, F, F, F, F, F) {
    let t33553 = t33518 + t33552;
    let t33554 = t113 * t33553;
    let t33555 = t8607 * t7756;
    let t33556 = t1442 * t8595;
    let t33583 = F::cast_from(2.0_f64) * t27188 * t1873;
    let t33585 = F::cast_from(2.0_f64) * t33234 * t1873;
    let t33587 = F::cast_from(2.0_f64) * t7042 * t7467;
    (t33553, t33554, t33555, t33556, t33583, t33585, t33587)
}
