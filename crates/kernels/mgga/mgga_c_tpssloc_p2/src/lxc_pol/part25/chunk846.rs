//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 846/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk846<F: Float>(t10913: F, t4583: F, t4582: F, t4588: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1000: F, t1020: F, t1025: F, t10263: F, t1041: F, t1046: F, t10517: F, t10860: F, t10863: F, t10866: F, t10871: F, t10873: F, t10876: F, t10879: F, t10883: F, t10886: F, t10891: F, t10896: F, t10898: F, t10904: F, t10909: F, t3043: F, t3057: F, t3109: F, t3117: F, t3123: F, t3134: F) -> F {
    let t10914 = t4583 * t10913;
    let t10915 = t4582 * t10914;
    let t10918 = t4588 * t10913;
    let t10919 = t4582 * t10918;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10929 = F::new(19.0) / F::new(576.0) * t10517 * t1025 + t1020 * t10860 / F::new(3072.0) - t10863 * t1046 / F::new(144.0) + t10866 / F::new(1152.0) - t10871 / F::new(6912.0) - t10873 / F::new(216.0) - t10876 * t10879 / F::new(512.0) + t10883 * t10886 / F::new(3072.0) + t10891 * t3043 / F::new(192.0) - t10896 / F::new(1536.0) - t10898 * t1025 / F::new(96.0) - t3109 * t3123 / F::new(192.0) - t10904 * t3134 / F::new(96.0) + t10909 / F::new(1536.0) + t3117 * t3057 / F::new(1536.0) - t1041 * t10915 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t1041 * t10919 - t10923 / F::new(432.0) + F::new(11.0) / F::new(108.0) * t10263 * t1000 - t10927 / F::new(54.0);
    t10929
}
