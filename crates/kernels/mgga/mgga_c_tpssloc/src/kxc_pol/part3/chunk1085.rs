//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1085/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1085<F: Float>(t28: F, t12000: F, t1649: F, t2: F, t3711: F, t1302: F, t15956: F, t16: F, t3231: F, t3673: F, t5178: F, t5181: F, t584: F, t16002: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t16003 = t12000 * t1649;
    let t16006 = t3711 * t2;
    let t16016 = piecewise3(t29, 0.0, 8.0 / 27.0 * t16003 * t3673 + 8.0 / 9.0 * t16006 * t15956 - 2.0 / 9.0 * t5178 * t3231 - 4.0 / 3.0 * t1302 * t584 + 4.0 * t5181 * t16);
    let t16018 = t16002 / 2.0 + t16016 / 2.0;
    (t16018,)
}
