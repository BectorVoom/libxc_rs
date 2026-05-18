//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 909/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk909<F: Float>(t1090: F, t11789: F, t248: F, t1227: F, t486: F, t676: F, t1216: F, t1213: F, t11552: F, t221: F, t456: F, t1197: F, t698: F) -> (F, F, F, F, F) {
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11818 = t676 * t486;
    let t11820 = t248 * t11818 * t1216;
    let t11821 = t1213 * t11820;
    let t11832 = t221 * t11552;
    let t11834 = F::new(5.0) / F::new(1296.0) * t456 * t11832;
    let t11835 = t698 * t1197;
    (t11792, t11818, t11821, t11834, t11835)
}
