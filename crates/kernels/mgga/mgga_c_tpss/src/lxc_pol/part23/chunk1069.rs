//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1069/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1069<F: Float>(t1061: F, t4143: F, t1531: F, t2949: F, t2931: F, t4146: F, t2957: F, t4142: F, t1530: F, t9467: F, t1080: F, t4181: F, t1543: F, t2993: F, t2975: F, t4184: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12177 = t4143 * t1061;
    let t12180 = t1531 * t2949;
    let t12183 = t4146 * t2931;
    let t12186 = t4142 * t2957;
    let t12187 = t12186 * t1061;
    let t12190 = t4146 * t2949;
    let t12193 = t1530 * t9467;
    let t12194 = t12193 * t2931;
    let t12201 = t4181 * t1080;
    let t12204 = t1543 * t2993;
    let t12207 = t4184 * t2975;
    (t12177, t12180, t12183, t12187, t12190, t12194, t12201, t12204, t12207)
}
