//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1143/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143<F: Float>(t20234: F, t43070: F, t10236: F, t135: F, t21458: F, t973: F, t42841: F, t2986: F, t4514: F, t61189: F, t21446: F, t21510: F, t13779: F, t21126: F, t61250: F, t13847: F, t17794: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69519 = t43070 * t20234;
    let t69529 = t10236 * t20234;
    let t69540 = t973 * t135 * t21458;
    let t69548 = t42841 * t20234;
    let t69570 = t2986 * t61189 * t4514;
    let t69579 = t973 * t135 * t21446;
    let t69647 = t10236 * t21510;
    let t69683 = t2986 * t13779 * t21126;
    let t69686 = t2986 * t61250 * t4514;
    let t69691 = t2986 * t13847 * t17794;
    (t69519, t69529, t69540, t69548, t69570, t69579, t69647, t69683, t69686, t69691)
}
