//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1123/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1123<F: Float>(t1266: F, t1772: F, t1842: F, t18481: F, t18483: F, t18496: F, t18948: F, t18950: F, t18964: F, t18968: F, t18972: F, t18976: F, t18981: F, t18986: F, t18991: F, t18994: F, t18997: F, t3367: F, t3385: F, t538: F, t5737: F, t5739: F, t5921: F, t5925: F, t5930: F, t5933: F) -> (F,) {
    let t18999 = -2.0 * t1266 * t18950 - t1772 * t18997 - t1842 * t18481 + 4.0 * t18483 * t5925 + 2.0 * t18483 * t5930 - 4.0 * t18496 * t18968 + t18948 * t538 - 6.0 * t18964 * t5739 + 4.0 * t18972 * t5739 + 2.0 * t18976 * t5739 - 2.0 * t18981 * t5739 + 2.0 * t18986 * t5739 + t18991 * t5739 + t18994 * t5739 + 2.0 * t3367 * t5921 - t3385 * t5921 - 2.0 * t5737 * t5933;
    (t18999,)
}
