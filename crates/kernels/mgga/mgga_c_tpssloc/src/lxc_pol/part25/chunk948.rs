//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 948/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk948<F: Float>(t24063: F, t539: F, t22645: F, t225: F, t7192: F, t2091: F, t3887: F, t3911: F, t12021: F, t3888: F, t7179: F, t12030: F, t12033: F, t12444: F, t1375: F, t1386: F, t2092: F, t22639: F, t22650: F, t3758: F, t3882: F, t3889: F, t3912: F, t568: F, t7194: F, t7199: F, t7214: F) -> (F, F, F, F, F, F) {
    let t24064 = t539 * t24063;
    let t24071 = 0.16449340668482264365e-1 * t22645;
    let t24082 = t7192 * t225;
    let t24088 = t3887 * t2091 * t3911;
    let t24092 = t12021 * t2091 * t3888;
    let t24095 = t7179 * t225;
    let t24098 = t24064 * t568 + 4.0 * t3758 * t7199 + 4.0 * t3882 * t7199 + 0.6579736267392905746e-1 * t22639 - t24071 - 2.0 * t3758 * t7214 - 2.0 * t3882 * t7214 - t7194 * t3912 - t12030 * t2092 - t12033 * t2092 + 0.16449340668482264365e-1 * t22650 - 2.0 * t12444 * t2092 - 2.0 * t24082 * t1386 + 2.0 * t7194 * t3889 + 2.0 * t1375 * t24088 - 6.0 * t1375 * t24092 - 2.0 * t24095 * t1386;
    (t24064, t24082, t24088, t24092, t24095, t24098)
}
