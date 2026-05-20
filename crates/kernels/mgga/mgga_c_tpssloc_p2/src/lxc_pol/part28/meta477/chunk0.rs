//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1691/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1691<F: Float>(t25374: F, t25927: F, t1081: F, t1530: F, t28: F, t4303: F, t1649: F, t776: F, t868: F, t1307: F, t1845: F, t645: F, t72: F, t7431: F) -> (F, F, F, F, F, F, F) {
    let t25928 = t25927 * t25374;
    let t25930 = t1081 * t1530;
    let t25934 = t28 * t4303;
    let t25938 = t1649 * t776;
    let t25945 = t1649 * t868;
    let t25988 = t1845 * t1307;
    let t26009 = t72 * t7431 * t645;
    (t25928, t25930, t25934, t25938, t25945, t25988, t26009)
}
