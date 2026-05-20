//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1031/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1031<F: Float>(t25482: F, t25527: F, t25560: F, t25729: F, t1055: F, t23384: F, t7566: F, t23394: F, t4664: F, t6704: F, t1634: F, t6815: F) -> (F, F, F, F) {
    let t25731 = t25482 + t25527 + t25560 + t25729;
    let t25732 = t1055 * t25731;
    let t25736 = t23384 * t7566;
    let t25738 = t23394 * t4664;
    let t25739 = t6704 * t25738;
    let t25742 = t6815 * t1634;
    (t25732, t25736, t25739, t25742)
}
