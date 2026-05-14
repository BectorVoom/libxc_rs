//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 878/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk878<F: Float>(t259: F, t479: F, t6032: F, t6521: F, t1885: F, t452: F, t6509: F, t1587: F, t1884: F, t1887: F, t473: F, t6019: F, t6024: F, t6031: F, t6510: F, t6514: F, t6517: F, t1153: F, t1589: F, t198: F, t330: F, t4023: F, t6044: F, t6200: F) -> (F, F, F, F) {
    let t480 = t259 < t479;
    let t6522 = t6032 * t6521;
    let t6525 = t1885 * t452 * t6509;
    let t6527 = -t1587 * t6019 - t1884 * t6525 - t1887 * t6514 + t473 * t6510 + 2.0 * t6024 * t6517 - t6031 * t6522;
    let t6534 = piecewise3(t480, t1153 * t198 * t330 * t6527 - t1589 * t4023 * t6044, t6200);
    (t6522, t6525, t6527, t6534)
}
