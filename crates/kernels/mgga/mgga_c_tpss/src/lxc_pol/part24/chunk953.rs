//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 953/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk953<F: Float>(t1270: F, t4519: F, t2222: F, t4435: F, t4377: F, t72: F, t732: F, t1173: F, t4432: F, t1613: F, t2331: F, t489: F, t9913: F, t123: F, t2349: F, t1614: F, t3305: F) -> (F, F, F, F, F, F, F, F) {
    let t12673 = t4519 * t1270;
    let t12677 = t4435 * t2222;
    let t12686 = t4377 * t72;
    let t12688 = 0.36622894612013090108e-3 * t12686 * t732;
    let t12689 = t1173 * t4432;
    let t12691 = t1613 * t2331;
    let t12692 = t489 * t12691;
    let t12742 = 32.0 * t9913;
    let t12743 = t1613 * t123;
    let t12744 = t12743 * t2349;
    let t12749 = t3305 * t1614;
    (t12673, t12677, t12688, t12689, t12692, t12742, t12744, t12749)
}
