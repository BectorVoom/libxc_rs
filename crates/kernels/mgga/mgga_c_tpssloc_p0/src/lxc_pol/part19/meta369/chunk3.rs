//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1363/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1363<F: Float>(t3061: F, t676: F, t1041: F, t248: F, t2771: F, t3129: F, t42742: F, t10962: F, t3103: F, t3078: F, t3082: F, t3089: F) -> (F, F, F, F, F) {
    let t43338 = t676 * t3061;
    let t43341 = t1041 * t248 * t43338 * t2771;
    let t43343 = t42742 * t3129;
    let t43350 = t10962 * t3103;
    let t43352 = t3078 * t3082;
    let t43354 = t3089 * t3082;
    (t43341, t43343, t43350, t43352, t43354)
}
