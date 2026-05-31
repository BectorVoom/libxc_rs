//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 646/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk646<F: Float>(t3426: F, t3972: F, t3931: F, t2464: F, t969: F, t3758: F, t970: F, t242: F, t1471: F, t2652: F, t2660: F, t2678: F, t2731: F, t2740: F, t2748: F, t2754: F, t3952: F, t3956: F, t3963: F, t3970: F, t946: F, t967: F) -> (F, F, F, F) {
    let t3973 = t3972 * t3426;
    let t3974 = t3931 * t3973;
    let t3977 = t969 * t2464;
    let t3978 = t3977 * t3426;
    let t3979 = t3931 * t3978;
    let t3982 = t970 * t3758;
    let t3983 = t242 * t3982;
    let t3986 = t946 * t3952 / F::cast_from(3072.0_f64) - t2731 * t3956 / F::cast_from(3072.0_f64) + t2678 / F::cast_from(4608.0_f64) - t2660 / F::cast_from(864.0_f64) - t2652 + t2754 / F::cast_from(6912.0_f64) + t2740 * t3963 / F::cast_from(4608.0_f64) - t2748 * t1471 / F::cast_from(864.0_f64) + t3970 / F::cast_from(6912.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t967 * t3974 - t967 * t3979 / F::cast_from(2304.0_f64) + t967 * t3983 / F::cast_from(4608.0_f64);
    (t3973, t3977, t3978, t3986)
}
