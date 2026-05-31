//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 927/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk927<F: Float>(t10913: F, t4583: F, t4582: F, t4588: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1000: F, t1020: F, t1025: F, t10263: F, t1041: F, t1046: F, t10517: F, t10860: F, t10863: F, t10866: F, t10871: F, t10873: F, t10876: F, t10879: F, t10883: F, t10886: F, t10891: F, t10896: F, t10898: F, t10904: F, t10909: F, t3043: F, t3057: F, t3109: F, t3117: F, t3123: F, t3134: F) -> F {
    let t10914 = t4583 * t10913;
    let t10915 = t4582 * t10914;
    let t10918 = t4588 * t10913;
    let t10919 = t4582 * t10918;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10929 = F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t10517 * t1025 + t1020 * t10860 / F::cast_from(3072.0_f64) - t10863 * t1046 / F::cast_from(144.0_f64) + t10866 / F::cast_from(1152.0_f64) - t10871 / F::cast_from(6912.0_f64) - t10873 / F::cast_from(216.0_f64) - t10876 * t10879 / F::cast_from(512.0_f64) + t10883 * t10886 / F::cast_from(3072.0_f64) + t10891 * t3043 / F::cast_from(192.0_f64) - t10896 / F::cast_from(1536.0_f64) - t10898 * t1025 / F::cast_from(96.0_f64) - t3109 * t3123 / F::cast_from(192.0_f64) - t10904 * t3134 / F::cast_from(96.0_f64) + t10909 / F::cast_from(1536.0_f64) + t3117 * t3057 / F::cast_from(1536.0_f64) - t1041 * t10915 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1041 * t10919 - t10923 / F::cast_from(432.0_f64) + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t10263 * t1000 - t10927 / F::cast_from(54.0_f64);
    t10929
}
