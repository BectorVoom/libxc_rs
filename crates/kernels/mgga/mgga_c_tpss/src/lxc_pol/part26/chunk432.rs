//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 432/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk432<F: Float>(t1415: F, t847: F, t854: F, t1407: F, t861: F, t141: F, t1409: F, t852: F, t860: F, t866: F) -> (F, F, F, F, F, F) {
    let t1416 = t847 * t1415;
    let t1419 = t854 * t1415;
    let t1421 = t861 * t1407;
    let t1422 = t141 * t1421;
    let t1424 = 0.1898925e1 * t1416 - t852 - 0.29896666666666666667e0 * t1409 + 0.3071625e0 * t1419 - t860 - 0.82156666666666666667e-1 * t1422;
    let t1425 = t1424 * t866;
    (t1416, t1419, t1421, t1422, t1424, t1425)
}
