//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1005/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1005<F: Float>(t10165: F, t13735: F, t1065: F, t4693: F, t3174: F, t2970: F, t4343: F, t973: F, t1611: F, t3088: F, t1036: F, t4617: F, t1023: F, t4347: F, t3071: F, t10422: F, t4574: F) -> (F, F, F, F, F, F, F) {
    let t13736 = t10165 * t13735;
    let t13742 = t4693 * t1065;
    let t13743 = t3174 * t13742;
    let t13748 = t2970 * t4343;
    let t13750 = t973 * t13748 / 216.0;
    let t13751 = t1611 * t3088;
    let t13758 = t4617 * t1036 / 2304.0;
    let t13761 = t4347 * t1023;
    let t13762 = t3071 * t13761;
    let t13765 = t10422 * t4574;
    (t13736, t13743, t13750, t13751, t13758, t13762, t13765)
}
