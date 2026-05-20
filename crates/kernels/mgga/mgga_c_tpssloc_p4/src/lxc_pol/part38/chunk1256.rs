//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1256/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1256<F: Float>(t28: F, t12072: F, t1649: F, t2: F, t3672: F, t1081: F, t584: F, t16: F, t3231: F, t3673: F, t5142: F, t5145: F, t517: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t15952 = t12072 * t1649;
    let t15955 = t3672 * t2;
    let t15956 = t584 * t1081;
    let t15966 = piecewise3::<F>(t29, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15952 * t3673 - F::new(16.0) / F::new(9.0) * t15955 * t15956 + F::new(4.0) / F::new(9.0) * t5142 * t3231 - F::new(8.0) / F::new(3.0) * t517 * t584 + F::new(8.0) * t5145 * t16);
    (t15956, t15966)
}
