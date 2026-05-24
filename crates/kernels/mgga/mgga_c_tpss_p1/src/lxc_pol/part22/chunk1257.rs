//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1257/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1257<F: Float>(t1232: F, t520: F, t6419: F, t5745: F, t1773: F, t20154: F, t522: F, t1772: F, t18483: F, t18496: F, t19540: F, t20179: F, t20183: F, t20187: F, t20191: F, t20196: F, t20200: F, t20202: F, t20206: F, t5737: F, t5739: F, t6430: F, t6433: F) -> (F, F, F) {
    let t20210 = t6419 * t1232 * t520;
    let t20211 = t5745 * t20210;
    let t20214 = t1773 * t522 * t20154;
    let t20216 = -t1772 * t20214 + t18483 * t6430 - F::new(2.0) * t18496 * t20187 - F::new(2.0) * t19540 * t20191 + t19540 * t20202 + F::new(2.0) * t20179 * t5739 + F::new(2.0) * t20183 * t5739 + t20196 * t5739 + t20200 * t5739 + F::new(2.0) * t20206 * t5739 + t20211 * t5739 - t5737 * t6433;
    (t20211, t20214, t20216)
}
