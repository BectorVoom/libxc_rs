//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 725/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk725<F: Float>(t2225: F, t522: F, t2221: F, t2223: F, t2516: F, t521: F, t17: F, t1284: F, t750: F, t1285: F, t592: F, t1287: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3819 = F::cast_from(20.0_f64) * t2225 * t522;
    let t3821 = F::cast_from(12.0_f64) * t2221 * t522;
    let t3823 = F::cast_from(32.0_f64) * t2223 * t522;
    let t3824 = t521 * t2516;
    let t3825 = t17 * t3824;
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3828 = F::cast_from(2.0_f64) * t3827;
    let t3829 = t592 * t1285;
    let t3830 = F::cast_from(8.0_f64) * t3829;
    let t3832 = F::cast_from(8.0_f64) * t592 * t1287;
    (t3819, t3821, t3823, t3824, t3825, t3826, t3827, t3828, t3829, t3830, t3832)
}
