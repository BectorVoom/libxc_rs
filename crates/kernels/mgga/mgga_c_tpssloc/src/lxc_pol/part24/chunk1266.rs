//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1266/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1266<F: Float>(t10160: F, t10358: F, t11007: F, t11010: F, t1920: F, t1945: F, t225: F, t23346: F, t23372: F, t23399: F, t3020: F, t3176: F, t345: F, t387: F, t388: F, t6687: F, t6691: F, t6695: F, t6768: F, t6816: F, t82382: F, t83420: F, t83424: F, t83435: F, t83441: F, t83444: F, t83453: F, t83457: F, t83459: F, t986: F) -> (F,) {
    let t83461 = -6.0 * t10160 * t6816 + 0.49348022005446793095e-1 * t6687 * t986 * t83420 + 0.82246703342411321826e-2 * t6687 * t83424 * t6691 + 6.0 * t23372 * t3176 + 3.0 * t3020 * t6768 * t388 + t10358 * t1945 * t388 - 0.82246703342411321826e-2 * t83435 + 0.65797362673929057459e-1 * t23346 * t23399 - 0.24125699647107321069e0 * t82382 * t6695 - 0.14621636149762012769e-1 * t83441 - 0.54831135561607547884e-2 * t83444 + 0.82246703342411321825e-2 * t1920 * t345 * t11007 * t225 * t387 - 3.0 * t11010 * t6816 + 0.24674011002723396548e-1 * t6687 * t986 * t83453 - 0.82246703342411321826e-2 * t83457 + 0.54831135561607547884e-2 * t83459;
    (t83461,)
}
