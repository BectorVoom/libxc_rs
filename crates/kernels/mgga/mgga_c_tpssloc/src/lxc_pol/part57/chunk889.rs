//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 889/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk889<F: Float>(t33363: F, t7754: F, t2018: F, t26161: F, t26558: F, t6463: F, t22574: F, t24432: F, t6347: F, t2035: F, t5493: F, t1874: F, t33234: F, t7461: F, t33617: F, t4028: F) -> (F, F, F, F, F, F, F) {
    let t128393 = 2.0 * t33363 * t7754;
    let t128397 = 2.0 * t26161 * t26558 * t2018 * t6463;
    let t128401 = 3.0 * t22574 * t24432 * t2018 * t6347;
    let t128402 = t2035 * t5493;
    let t128404 = 2.0 * t128402 * t1874;
    let t128406 = 4.0 * t33234 * t7461;
    let t128413 = 4.0 * t4028 * t33617;
    (t128393, t128397, t128401, t128402, t128404, t128406, t128413)
}
