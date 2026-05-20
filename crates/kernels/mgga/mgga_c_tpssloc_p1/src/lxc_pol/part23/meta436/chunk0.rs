//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1277/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277<F: Float>(t11697: F, t22153: F, t3577: F, t13969: F, t22274: F, t3515: F, t1227: F, t22196: F, t1222: F, t22015: F, t20246: F, t972: F) -> (F, F, F, F, F) {
    let t73084 = t3577 * t11697 * t22153;
    let t73096 = t3515 * t13969 * t22274;
    let t73099 = t1227 * t13969 * t22196;
    let t73102 = t22015 * t1222;
    let t73113 = t20246 * t972;
    (t73084, t73096, t73099, t73102, t73113)
}
