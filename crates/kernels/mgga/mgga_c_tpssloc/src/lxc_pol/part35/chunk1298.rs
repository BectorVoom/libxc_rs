//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1298/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1298<F: Float>(t103291: F, t103293: F, t103304: F, t103315: F, t109206: F, t109244: F, t109283: F, t109324: F, t109356: F, t109393: F, t109432: F, t109732: F, t1238: F, t1241: F, t1716: F, t1760: F, t19234: F, t21510: F, t22003: F, t24589: F, t24601: F, t24602: F, t24615: F, t27382: F, t27444: F, t27784: F, t27785: F, t27820: F, t29536: F, t29812: F, t4945: F, t5398: F, t6146: F, t7283: F, t7300: F, t8061: F, t8088: F, t94525: F) -> (F,) {
    let t109743 = 0.49348022005446793095e-1 * t7283 * t7300 * t24615 * t22003 - 0.82246703342411321826e-2 * t103291 + 0.24674011002723396548e-1 * t7283 * t1716 * t103315 + 12.0 * t19234 * t8061 + 0.36554090374405031922e-2 * t103293 + 0.82246703342411321826e-2 * t24589 * t24601 * t24602 * t5398 * t1760 + 0.18277045187202515961e-2 * t94525 + 0.24674011002723396548e-1 * t7283 * t6146 * t27382 + 0.82246703342411321826e-2 * t24589 * t27820 * t29812 - 0.16449340668482264365e-1 * t24589 * t24601 * t27444 * t21510 - 0.16449340668482264365e-1 * t103304 - 6.0 * t19234 * t8088 - t1238 * t1241 * (t109206 + t109244 + t109283 + t109324 + t109356 + t109393 + t109432 + t109732) - 18.0 * t27784 * t27785 * t22003 + 6.0 * t4945 * t29536;
    (t109743,)
}
