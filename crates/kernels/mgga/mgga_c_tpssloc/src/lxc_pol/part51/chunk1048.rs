//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1048/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1048<F: Float>(t31236: F, t5113: F, t8326: F, t191: F, t192: F, t6872: F, t3938: F, t671: F, t3941: F, t6880: F, t8607: F, t2095: F, t31035: F, t1983: F, t6999: F, t8640: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31237 = 2.0 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0 * t31238;
    let t31246 = t6872 * t191 * t192;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0 * t31286;
    let t31294 = 3.0 * t8607 * t6880;
    let t31295 = t2095 * t31035;
    let t31296 = t1983 * t31295;
    let t31297 = t8640 * t6999;
    (t31237, t31239, t31246, t31284, t31285, t31287, t31294, t31295, t31296, t31297)
}
