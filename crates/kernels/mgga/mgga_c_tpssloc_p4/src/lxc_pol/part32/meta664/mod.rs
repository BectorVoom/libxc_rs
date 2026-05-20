//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta664<F: Float>(t24574: F, t27779: F, t8015: F, t85660: F, t27826: F, t27403: F, t27389: F, t8074: F, t85917: F, t24826: F, t27511: F, t15394: F, t2127: F, t221: F, t11147: F, t491: F, t1089: F, t1751: F, t7327: F, t1653: F, t7330: F, t85822: F, t131: F, t1419: F, t23598: F, t467: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095::<F>(t24574, t27779, t8015, t85660, t27826, t27403, t27389, t8074, t85917, t24826, t27511, t15394, t2127, t221);
        let (t94797, t94837, t94847, t94858) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2096::<F>(t11147, t491, t1089, t1751, t7327, t1653, t7330, t85822, t131, t1419, t23598, t467);
    (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796, t94797, t94837, t94847, t94858)
}
