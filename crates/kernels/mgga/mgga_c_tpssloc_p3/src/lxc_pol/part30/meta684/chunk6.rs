//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2159/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2159<F: Float>(t1339: F, t22827: F, t550: F, t74366: F, t1307: F, t6415: F, t6420: F, t1825: F, t5286: F, t6936: F, t57091: F, t91144: F, t91155: F, t91159: F, t91162: F, t91171: F, t91180: F, t93650: F, t93656: F, t97273: F, t97277: F, t97281: F, t97283: F, t97287: F) -> F {
    let t97291 = t22827 * t1339 * t74366 * t550;
    let t97295 = t22827 * t1339 * t6415 * t1307;
    let t97299 = t22827 * t1339 * t6420 * t1307;
    let t97303 = t6936 * t1339 * t1825 * t5286;
    let t97307 = t6936 * t1339 * t57091 * t550;
    let t97309 = -t91144 + F::cast_from(0.24223653656484234512e-2_f64) * t97273 + F::cast_from(0.24223653656484234512e-2_f64) * t97277 - F::cast_from(0.24223653656484234512e-2_f64) * t97281 - t93650 + t91155 - t91159 + t91162 - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t97283 - F::cast_from(0.84782787797694820792e-2_f64) * t97287 + F::cast_from(0.12111826828242117256e-2_f64) * t97291 + F::cast_from(0.12111826828242117256e-2_f64) * t97295 + F::cast_from(0.12111826828242117256e-2_f64) * t97299 - F::cast_from(0.40372756094140390854e-3_f64) * t97303 - F::cast_from(0.20186378047070195427e-3_f64) * t97307 - t93656 - t91171 - t91180;
    t97309
}
