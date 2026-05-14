//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1343/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1343<F: Float>(t20112: F, t4674: F, t547: F, t18592: F, t1668: F, t20116: F, t1279: F, t21569: F, t21560: F, t16064: F, t16076: F, t1670: F, t1784: F, t20094: F, t4556: F, t4559: F, t6284: F, t71074: F, t71076: F, t71085: F, t71087: F, t71091: F, t71093: F, t71097: F, t71100: F) -> (F,) {
    let t71103 = 6.0 * t547 * t20112 * t4674;
    let t71106 = 6.0 * t547 * t18592 * t4674;
    let t71108 = 12.0 * t1668 * t20116;
    let t71110 = 3.0 * t1279 * t21569;
    let t71112 = 6.0 * t1279 * t21560;
    let t71115 = 6.0 * t16064 * t1784 + 3.0 * t16076 * t1784 + 6.0 * t1670 * t20094 + 12.0 * t4556 * t6284 + 6.0 * t4559 * t6284 + t71074 + t71076 + t71085 + t71087 + t71091 + t71093 + t71097 + t71100 + t71103 + t71106 + t71108 + t71110 + t71112;
    (t71115,)
}
