//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1180/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1180<F: Float>(t1982: F, t77: F, t84: F, t1981: F, t582: F, t1679: F, t619: F, t1678: F, t615: F, t2049: F, t1985: F, t578: F, t1993: F, t1675: F, t1680: F, t18305: F, t18325: F, t18328: F, t18332: F, t18335: F, t18338: F, t18342: F, t18345: F, t5483: F, t5487: F, t5489: F, t5492: F, t5503: F, t5507: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18347 = t77 * t84 * t1982;
    let t18350 = t1981 * t582;
    let t18351 = t1679 * t619;
    let t18352 = t1678 * t18351;
    let t18356 = t77 * t615 * t619;
    let t18360 = t77 * t84 * t2049;
    let t18363 = t578 * t1985;
    let t18366 = t578 * t1993;
    let t18373 = -t18305 * t1680 / 6.0 - t5483 * t5503 / 3.0 - t5483 * t5507 / 3.0 - t1675 * t18325 / 6.0 - t1675 * t18328 / 3.0 - t1675 * t18332 / 6.0 + 5.0 / 3.0 * t18335 * t5489 + 2.0 / 3.0 * t18338 * t1680 + 5.0 / 3.0 * t18342 * t5489 - 5.0 * t18345 * t18347 - 10.0 / 3.0 * t18350 * t18352 + 5.0 / 3.0 * t5487 * t18356 + 5.0 / 6.0 * t5487 * t18360 + t18363 * t1680 / 3.0 + t18366 * t1680 / 3.0 + 2.0 / 3.0 * t5492 * t5503 + 2.0 / 3.0 * t5492 * t5507;
    (t18347, t18350, t18351, t18352, t18356, t18360, t18363, t18366, t18373)
}
