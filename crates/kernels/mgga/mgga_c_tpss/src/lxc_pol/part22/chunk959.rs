//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 959/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk959<F: Float>(t498: F, t7622: F, t14: F, t563: F, t3297: F, t724: F, t489: F, t1193: F, t8038: F, t2206: F, t3178: F, t1184: F, t3305: F) -> (F, F, F, F, F, F) {
    let t10019 = F::new(24.0) * t7622 * t498;
    let t10021 = t14 * t563;
    let t10022 = t10021 * t498;
    let t10024 = t3297 * t724;
    let t10025 = t489 * t10024;
    let t10028 = F::new(0.10254018858216406658e4) * t1193 * t8038;
    let t10029 = t3178 * t2206;
    let t10033 = t3305 * t1184;
    (t10019, t10022, t10025, t10028, t10029, t10033)
}
