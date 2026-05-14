//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1024/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1024<F: Float>(t26361: F, t225: F, t7919: F, t2085: F, t5210: F, t1824: F, t5250: F, t1352: F, t26393: F, t1825: F, t24116: F, t26406: F, t1336: F, t22707: F, t24099: F, t26379: F, t26381: F, t26386: F, t26390: F, t26398: F, t26412: F, t26416: F, t26419: F, t26424: F, t26427: F, t3777: F, t5234: F, t5334: F, t5344: F, t7209: F, t7932: F) -> (F, F, F, F, F, F, F) {
    let t27067 = 0.38381794893125283518e-1 * t26361;
    let t27068 = t7919 * t225;
    let t27070 = t5210 * t2085;
    let t27074 = t2085 * t1824;
    let t27075 = t27074 * t5250;
    let t27078 = t27074 * t1352;
    let t27082 = 0.16449340668482264365e-1 * t26393;
    let t27086 = t24116 * t1825;
    let t27088 = 0.38381794893125283518e-1 * t26406;
    let t27095 = 0.3289868133696452873e-1 * t26379 + 0.76763589786250567037e-1 * t26381 + 2.0 * t5334 * t27075 - t24099 - t5344 * t27078 - 0.3289868133696452873e-1 * t26386 - 0.3289868133696452873e-1 * t26390 + t27082 - 0.3289868133696452873e-1 * t26398 - t5234 * t7209 - t3777 * t7932 - t1336 * t27086 + t27088 + 0.82246703342411321825e-2 * t22707 - 0.16449340668482264365e-1 * t26412 + 0.3289868133696452873e-1 * t26416 - 0.16449340668482264365e-1 * t26419 + 0.3289868133696452873e-1 * t26424 + 0.82246703342411321825e-2 * t26427;
    (t27067, t27068, t27070, t27074, t27075, t27078, t27095)
}
