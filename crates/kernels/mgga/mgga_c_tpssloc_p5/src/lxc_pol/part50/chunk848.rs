//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 848/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk848<F: Float>(t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t43: F, t614: F, t2267: F, t38: F, t33: F, t6504: F) -> (F, F, F, F, F, F, F, F) {
    let t22468 = t240 * t107;
    let t22469 = F::new(11.0) / F::new(9.0) * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22473 = t63 * t2331;
    let t22502 = t614 * t43;
    let t22505 = t38 * t2267;
    let t22510 = F::new(88.0) / F::new(9.0) * t240;
    let t22522 = t33 * t6504;
    (t22469, t22470, t22471, t22473, t22502, t22505, t22510, t22522)
}
