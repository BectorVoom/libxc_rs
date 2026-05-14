//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1161/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1161<F: Float>(t1880: F, t3048: F, t1107: F, t6016: F, t3154: F, t6040: F, t1889: F, t9519: F, t38: F, t5974: F, t1981: F, t2016: F, t55: F, t5965: F, t7682: F, t7690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19144 = t3048 * t1880;
    let t19150 = t1107 * t6016;
    let t19168 = t6040 * t3154;
    let t19172 = t1889 * t9519;
    let t19191 = t38 * t5974;
    let t19192 = t1981 * t19191;
    let t19213 = t55 * t2016;
    let t19229 = t7682 * t5965;
    let t19232 = t7690 * t5965;
    (t19144, t19150, t19168, t19172, t19191, t19192, t19213, t19229, t19232)
}
