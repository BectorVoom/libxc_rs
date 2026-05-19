//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1118/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1118<F: Float>(t17157: F, t4510: F, t17161: F, t13798: F, t17152: F, t10236: F, t5392: F, t10235: F, t13851: F, t4514: F, t10287: F, t10333: F, t10339: F, t13893: F, t13896: F, t13907: F, t13909: F, t13915: F, t2986: F) -> F {
    let t17854 = t4510 * t17157;
    let t17857 = t4510 * t17161;
    let t17860 = t13798 * t17152;
    let t17863 = t10236 * t5392;
    let t17864 = t10235 * t17863;
    let t17867 = t13851 * t4514;
    let t17873 = -t13893 - F::cast_from(0.12345679012345679012e-3_f64) * t13896 - F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t17854 + F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t17857 + F::cast_from(0.86419753086419753084e-3_f64) * t2986 * t17860 - F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t17864 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17867 + F::cast_from(0.18518518518518518518e-3_f64) * t10287 + F::cast_from(0.49382716049382716048e-3_f64) * t10333 + t10339 + t13907 + F::cast_from(0.37037037037037037036e-3_f64) * t13909 - t13915;
    t17873
}
