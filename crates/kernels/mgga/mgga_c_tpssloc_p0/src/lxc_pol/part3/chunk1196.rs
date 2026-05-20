//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1196/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1196<F: Float>(t13969: F, t4988: F, t1227: F, t15708: F, t4723: F, t11668: F, t1725: F, t698: F, t1174: F, t1230: F, t14706: F, t248: F) -> (F, F, F, F) {
    let t15743 = t13969 * t4988;
    let t15745 = F::new(5.0) / F::new(10368.0) * t1227 * t15743;
    let t15749 = t4723 * t15708;
    let t15750 = t11668 * t15749;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15761 = t248 * t1230 * t14706;
    (t15745, t15750, t15754, t15761)
}
