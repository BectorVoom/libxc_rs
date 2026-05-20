//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1806/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1806<F: Float>(t13861: F, t2988: F, t13542: F, t4518: F, t13546: F, t10259: F, t4514: F, t13559: F, t13555: F, t4510: F, t1597: F, t3014: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t13862 = t2988 * t13861;
    let t13865 = t4518 * t13542;
    let t13868 = t4518 * t13546;
    let t13871 = t10259 * t4514;
    let t13874 = t4518 * t13559;
    let t13877 = t4510 * t13555;
    let t13881 = t1597 * t3014 * t343;
    (t13862, t13865, t13868, t13871, t13874, t13877, t13881)
}
