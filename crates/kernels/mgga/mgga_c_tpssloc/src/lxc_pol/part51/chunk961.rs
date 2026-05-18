//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 961/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk961<F: Float>(t1921: F, t23587: F, t3034: F, t38: F, t131: F, t350: F, t3030: F, t344: F, t225: F, t6733: F, t1949: F, t2966: F) -> (F, F, F, F, F) {
    let t23588 = t1921 * t23587;
    let t23598 = F::new(1.0) / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    let t23601 = t23600 * t350;
    let t23602 = t344 * t3030;
    let t23613 = t6733 * t225;
    let t23617 = t2966 * t1949;
    (t23588, t23601, t23602, t23613, t23617)
}
