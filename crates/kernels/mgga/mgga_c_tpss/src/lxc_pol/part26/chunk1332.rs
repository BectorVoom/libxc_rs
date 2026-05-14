//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1332/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1332<F: Float>(t4637: F, t547: F, t5531: F, t20112: F, t4674: F, t18592: F, t1668: F, t20116: F, t1279: F, t21569: F, t21560: F, t16064: F, t16073: F, t1670: F, t1904: F, t20997: F, t5474: F, t6067: F, t71032: F, t71037: F, t71041: F, t71043: F, t71045: F, t71049: F, t71057: F, t71059: F, t71063: F, t71067: F, t71070: F, t71074: F, t71076: F) -> (F, F, F, F, F, F, F) {
    let t71100 = 6.0 * t547 * t4637 * t5531;
    let t71103 = 6.0 * t547 * t20112 * t4674;
    let t71106 = 6.0 * t547 * t18592 * t4674;
    let t71108 = 12.0 * t1668 * t20116;
    let t71110 = 3.0 * t1279 * t21569;
    let t71112 = 6.0 * t1279 * t21560;
    let t72768 = 6.0 * t16064 * t1904 + 6.0 * t16073 * t1904 + 6.0 * t1670 * t20997 + 6.0 * t5474 * t6067 + t71032 + t71037 + t71041 + t71043 + t71045 + t71049 + t71057 + t71059 + t71063 + t71067 + t71070 + t71074 + t71076;
    (t71100, t71103, t71106, t71108, t71110, t71112, t72768)
}
