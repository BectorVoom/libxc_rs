//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 897/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk897<F: Float>(t1242: F, t2376: F, t339: F, t1250: F, t3342: F, t3354: F, t1184: F, t3211: F, t498: F, t7622: F, t14: F, t563: F, t3297: F, t724: F, t489: F, t1193: F, t8038: F) -> (F, F, F, F, F, F, F, F) {
    let t9994 = t339 * t1242 * t2376;
    let t9995 = t9994 * t1250;
    let t9997 = t3342 * t3354;
    let t10016 = t3211 * t1184;
    let t10019 = 24.0 * t7622 * t498;
    let t10021 = t14 * t563;
    let t10022 = t10021 * t498;
    let t10024 = t3297 * t724;
    let t10025 = t489 * t10024;
    let t10028 = 0.10254018858216406658e4 * t1193 * t8038;
    (t9994, t9995, t9997, t10016, t10019, t10022, t10025, t10028)
}
