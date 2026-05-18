//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 978/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk978<F: Float>(t10498: F, t187: F, t8043: F, t1398: F, t8096: F, t8077: F, t123: F, t1354: F, t2349: F, t10470: F, t10471: F, t10472: F, t1692: F, t2133: F, t2433: F, t2439: F, t3548: F, t7929: F, t7932: F, t7936: F, t7945: F, t8000: F, t8001: F, t8019: F, t8023: F, t8029: F, t8040: F) -> (F, F, F, F, F, F) {
    let t10500 = F::new(0.19751673498613801407e-1) * t10498 * t187;
    let t10501 = F::new(12.0) * t8043;
    let t10502 = t1398 * t8096;
    let t10509 = F::new(8.0) * t8077;
    let t10510 = t1354 * t123;
    let t10511 = t10510 * t2349;
    let t10512 = F::new(0.10843581300301739842e-1) * t10511;
    let t10513 = F::new(2.0) * t10502 * t1692 * t2433 + F::new(3.0) * t2133 * t2439 * t3548 - t10470 + t10471 - t10472 + t10500 + t10501 + t10509 + t10512 + t7929 - t7932 - t7936 + t7945 + t8000 + t8001 - t8019 + t8023 - t8029 - t8040;
    (t10500, t10501, t10502, t10509, t10512, t10513)
}
