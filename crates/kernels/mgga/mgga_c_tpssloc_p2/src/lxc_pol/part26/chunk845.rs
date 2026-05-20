//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 845/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk845<F: Float>(t761: F, t9919: F, t2531: F, t2535: F, t2427: F, t2430: F, t185: F, t9258: F, t707: F, t32: F, t717: F, t2659: F) -> (F, F, F, F, F) {
    let t9921 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9923 = F::cast_from(0.17544670867903938621e1_f64) * t9922;
    let t9924 = t2427 * t2430;
    let t9925 = F::new(24.0) * t9924;
    let t9926 = t185 * t9258;
    let t9928 = F::new(4.0) * t707 * t9926;
    let t9929 = t32 * t717;
    let t9931 = F::new(36.0) * t9929 * t2659;
    (t9921, t9923, t9925, t9928, t9931)
}
