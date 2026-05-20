//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1817;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta548<F: Float>(t23196: F, t23204: F, t6562: F, t6556: F, t81632: F, t23012: F, t6573: F, t1883: F, t82045: F, t23164: F, t6555: F, t82133: F, t23197: F, t6547: F, t23257: F, t794: F, t6568: F, t23205: F, t82038: F, t23242: F, t81979: F, t1081: F, t2752: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82182, t82209, t82211, t82218, t82221) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1817::<F>(t23196, t23204, t6562, t6556, t81632, t23012, t6573, t1883, t82045, t23164, t6555, t82133);
        let (t82230, t82236, t82259, t82294, t82296, t83555) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1818::<F>(t23197, t6547, t23257, t6562, t794, t23012, t6568, t23205, t82038, t23242, t81979, t1081, t2752);
    (t82182, t82209, t82211, t82218, t82221, t82230, t82236, t82259, t82294, t82296, t83555)
}
