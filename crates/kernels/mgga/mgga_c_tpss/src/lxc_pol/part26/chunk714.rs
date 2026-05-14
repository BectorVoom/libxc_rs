//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 714/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk714<F: Float>(t1270: F, t1659: F, t198: F, t507: F, t1625: F, t541: F, t3184: F, t1206: F, t1268: F, t2292: F, t2302: F, t2310: F, t3183: F, t3198: F, t3209: F, t3213: F, t3281: F, t3307: F, t3310: F, t4440: F, t4441: F, t4442: F, t4524: F, t4525: F) -> (F, F, F, F) {
    let t4528 = t1659 * t1270;
    let t4532 = t198 * t507;
    let t4533 = t541 * t1625;
    let t4537 = t3184 * t1625;
    let t4540 = 3.0 * t1206 * t3183 * t4528 + 6.0 * t1206 * t4532 * t4533 - t1268 * t4524 * t4525 + 3.0 * t3183 * t4537 - t2292 + t2302 + t2310 + t3198 - t3209 - t3213 + t3281 + t3307 + t3310 - t4440 - t4441 - t4442;
    (t4528, t4532, t4533, t4540)
}
