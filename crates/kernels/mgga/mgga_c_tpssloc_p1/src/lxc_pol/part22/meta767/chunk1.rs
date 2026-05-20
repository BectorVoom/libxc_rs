//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2593/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593<F: Float>(t11728: F, t22312: F, t248: F, t3570: F, t1174: F, t1177: F, t15495: F, t6221: F, t65552: F, t65554: F, t65558: F, t65567: F, t71189: F, t71201: F, t72273: F, t72285: F, t72287: F, t72289: F, t72293: F) -> F {
    let t72297 = t11728 * t248 * t3570 * t22312;
    let t72299 = t65552 / F::new(3456.0) + t65554 / F::new(1536.0) - t72273 / F::new(6912.0) - t65558 / F::new(2304.0) + t65567 / F::new(36.0) - t1174 * t1177 * t71201 / F::new(48.0) - t1174 * t1177 * t71189 / F::new(48.0) - t15495 * t6221 / F::new(192.0) - t72285 / F::new(1152.0) + t72287 / F::new(768.0) + t72289 / F::new(432.0) + t72293 / F::new(4608.0) - t72297 / F::new(768.0);
    t72299
}
