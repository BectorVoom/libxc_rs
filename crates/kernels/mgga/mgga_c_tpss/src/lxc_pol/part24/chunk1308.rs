//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1308/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1308<F: Float>(t14245: F, t19696: F, t215: F, t14311: F, t5559: F, t17974: F, t4775: F, t4771: F, t14193: F, t17964: F, t14197: F, t14202: F, t14212: F, t63920: F, t14216: F, t19703: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69948 = t19696 * t215 * t14245;
    let t69950 = t5559 * t14311;
    let t69952 = t17974 * t4775;
    let t69954 = t17974 * t4771;
    let t69956 = t17964 * t14193;
    let t69958 = t17964 * t14197;
    let t69960 = t17964 * t14202;
    let t69962 = t63920 * t14212;
    let t69964 = t19703 * t14216;
    (t69948, t69950, t69952, t69954, t69956, t69958, t69960, t69962, t69964)
}
