//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1300/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1300<F: Float>(t103413: F, t103494: F, t104502: F, t104635: F, t109722: F, t1761: F, t19232: F, t19249: F, t2124: F, t2155: F, t27406: F, t27426: F, t27830: F, t29554: F, t29798: F, t29812: F, t29816: F, t5055: F, t6244: F, t7283: F, t73891: F, t8061: F, t8088: F, t94701: F) -> (F,) {
    let t109809 = -0.82246703342411321826e-2 * t103413 + 6.0 * t27830 * t6244 - 0.3752886611772249944e0 * t109722 * t2124 + 6.0 * t19232 * t8061 - 3.0 * t73891 * t2155 - 3.0 * t19232 * t8088 - 0.82246703342411321826e-2 * t7283 * t27426 * t29812 - 0.16449340668482264365e-1 * t7283 * t27426 * t29816 + 0.13159472534785811492e0 * t27406 * t29554 - 18.0 * t5055 * t29798 + 0.54831135561607547884e-2 * t94701 - 0.16449340668482264365e-1 * t103494 - 3.0 * t19249 * t8088 - 3.0 * t104635 * t1761 + 0.16449340668482264365e-1 * t104502;
    (t109809,)
}
