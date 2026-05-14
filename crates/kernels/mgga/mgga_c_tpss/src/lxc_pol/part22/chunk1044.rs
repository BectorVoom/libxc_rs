//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1044/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1044<F: Float>(t1270: F, t4519: F, t2222: F, t4435: F, t1206: F, t1268: F, t4377: F, t72: F, t732: F, t1173: F, t4432: F, t1613: F, t2331: F, t489: F, t1288: F, t9856: F) -> (F, F, F, F, F, F, F) {
    let t12673 = t4519 * t1270;
    let t12677 = t4435 * t2222;
    let t12678 = 0.24415263074675393405e-3 * t12677;
    let t12679 = t1206 * t1268;
    let t12686 = t4377 * t72;
    let t12688 = 0.36622894612013090108e-3 * t12686 * t732;
    let t12689 = t1173 * t4432;
    let t12690 = 8.0 * t12689;
    let t12691 = t1613 * t2331;
    let t12692 = t489 * t12691;
    let t12696 = t9856 * t1288;
    (t12673, t12678, t12679, t12688, t12690, t12692, t12696)
}
