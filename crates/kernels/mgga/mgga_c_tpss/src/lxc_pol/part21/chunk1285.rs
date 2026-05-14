//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1285/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1285<F: Float>(t61063: F, t10587: F, t19703: F, t10610: F, t10614: F, t1369: F, t61062: F, t17974: F, t3689: F, t10623: F, t5559: F, t1385: F, t61086: F, t17946: F, t3622: F, t10632: F, t5547: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t63949 = 35.0 / 108.0 * t61063;
    let t63951 = t19703 * t10587;
    let t63953 = t19703 * t10610;
    let t63955 = t19703 * t10614;
    let t63957 = t61062 * t1369;
    let t63960 = t17974 * t3689;
    let t63961 = 7.0 / 288.0 * t63960;
    let t63962 = t5559 * t10623;
    let t63964 = t61086 * t1385;
    let t63966 = t17946 * t3622;
    let t63967 = 7.0 / 72.0 * t63966;
    let t63968 = t5547 * t10632;
    (t63949, t63951, t63953, t63955, t63957, t63961, t63962, t63964, t63967, t63968)
}
