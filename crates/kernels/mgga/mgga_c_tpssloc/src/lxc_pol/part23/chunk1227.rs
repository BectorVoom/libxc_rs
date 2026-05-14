//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1227/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227<F: Float>(t42110: F, t42113: F, t76637: F, t959: F, t17934: F, t5804: F, t5694: F, t42100: F, t42102: F, t5695: F, t60357: F, t21268: F, t49489: F, t10702: F, t2844: F, t1557: F, t68924: F) -> (F, F, F, F, F, F, F, F) {
    let t76641 = 0.91082604192152556044e5 * t959 * t42110 * t76637 * t42113;
    let t76643 = 0.70178683471615754484e1 * t17934 * t5804;
    let t76644 = t5694 * t5694;
    let t76647 = 0.24955700379505800916e5 * t42100 * t76644 * t42102;
    let t76652 = 12.0 * t60357 * t5695;
    let t76654 = 0.3859675079686208416e3 * t49489 * t21268;
    let t76657 = 0.57895126195293126241e3 * t10702 * t76644 * t2844;
    let t76659 = 4.0 * t68924 * t1557;
    (t76641, t76643, t76644, t76647, t76652, t76654, t76657, t76659)
}
