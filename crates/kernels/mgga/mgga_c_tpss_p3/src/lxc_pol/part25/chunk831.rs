//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 831/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk831<F: Float>(t1265: F, t1838: F, t5740: F, t1232: F, t520: F, t5745: F, t1773: F, t522: F, t5918: F, t1266: F, t1772: F, t1842: F, t538: F, t5737: F, t5739: F, t5919: F, t5921: F) -> (F, F, F, F) {
    let t5924 = t1838 * t1265;
    let t5925 = t5740 * t5924;
    let t5930 = t5745 * t1838 * t1232 * t520;
    let t5933 = t1773 * t522 * t5918;
    let t5935 = -t1266 * t5921 - t1772 * t5933 - t1842 * t5737 + t538 * t5919 + F::cast_from(2.0_f64) * t5739 * t5925 + t5739 * t5930;
    (t5925, t5930, t5933, t5935)
}
