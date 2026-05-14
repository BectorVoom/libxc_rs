//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1007/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1007<F: Float>(t26728: F, t4272: F, t2047: F, t4142: F, t1492: F, t7084: F, t13042: F, t13053: F, t13065: F, t2054: F, t23250: F, t23254: F, t24318: F, t24321: F, t25168: F, t25339: F, t25343: F, t259: F, t26722: F, t26726: F) -> (F, F, F, F) {
    let t26729 = t26728 * t4272;
    let t26732 = t4142 * t2047;
    let t26734 = t1492 * t7084;
    let t26737 = -t23250 + t24318 - 0.82246703342411321825e-2 * t23254 + t24321 - t13065 * t2054 + t26722 * t259 - 0.3289868133696452873e-1 * t25339 - 0.3289868133696452873e-1 * t25343 + t26726 - t13042 * t2054 - 6.0 * t25168 * t26729 + t26732 * t259 + t26734 * t259 - t13053 * t2054;
    (t26729, t26732, t26734, t26737)
}
