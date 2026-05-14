//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 667/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk667<F: Float>(t345: F, t3949: F, t947: F, t242: F, t3932: F, t949: F, t3931: F, t1407: F, t2741: F, t2751: F, t967: F, t2459: F, t2761: F, t3426: F, t2464: F, t969: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3950 = t3949 * t345;
    let t3951 = t947 * t3950;
    let t3952 = t242 * t3951;
    let t3955 = t3932 * t949;
    let t3956 = t3931 * t3955;
    let t3962 = t1407 * t949;
    let t3963 = t2741 * t3962;
    let t3968 = t2751 * t1407;
    let t3969 = t242 * t3968;
    let t3970 = t967 * t3969;
    let t3972 = t2761 * t2459;
    let t3973 = t3972 * t3426;
    let t3974 = t3931 * t3973;
    let t3977 = t969 * t2464;
    (t3950, t3952, t3955, t3956, t3962, t3963, t3969, t3970, t3972, t3973, t3974, t3977)
}
