//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1283/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1283<F: Float>(t3671: F, t61033: F, t61051: F, t1381: F, t61050: F, t61063: F, t1369: F, t61062: F, t17974: F, t3689: F, t1385: F, t61086: F) -> (F, F, F, F, F, F, F) {
    let t63928 = t61033 * t3671;
    let t63935 = F::new(119.0) / F::new(3456.0) * t61051;
    let t63945 = t61050 * t1381;
    let t63949 = F::new(35.0) / F::new(108.0) * t61063;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    let t63964 = t61086 * t1385;
    (t63928, t63935, t63945, t63949, t63957, t63960, t63964)
}
