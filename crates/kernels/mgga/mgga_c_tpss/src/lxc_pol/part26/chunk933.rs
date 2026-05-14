//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 933/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk933<F: Float>(t1242: F, t2376: F, t339: F, t1250: F, t1184: F, t3211: F, t498: F, t7622: F, t14: F, t563: F, t1193: F, t8038: F, t2206: F, t3178: F, t3214: F, t3305: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9994 = t339 * t1242 * t2376;
    let t9995 = t9994 * t1250;
    let t10016 = t3211 * t1184;
    let t10019 = 24.0 * t7622 * t498;
    let t10021 = t14 * t563;
    let t10022 = t10021 * t498;
    let t10028 = 0.10254018858216406658e4 * t1193 * t8038;
    let t10029 = t3178 * t2206;
    let t10031 = t3214 * t1184;
    let t10033 = t3305 * t1184;
    (t9994, t9995, t10016, t10019, t10022, t10028, t10029, t10031, t10033)
}
