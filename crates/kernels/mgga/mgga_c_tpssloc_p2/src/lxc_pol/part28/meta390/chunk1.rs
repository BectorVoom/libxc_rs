//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1520/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1520<F: Float>(t28: F, t12000: F, t1649: F, t2: F, t3711: F, t1302: F, t15956: F, t16: F, t3231: F, t3673: F, t5178: F, t5181: F, t584: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t16003 = t12000 * t1649;
    let t16006 = t3711 * t2;
    let t16016 = piecewise3::<F>(t29, F::new(0.0), F::new(8.0) / F::new(27.0) * t16003 * t3673 + F::new(8.0) / F::new(9.0) * t16006 * t15956 - F::new(2.0) / F::new(9.0) * t5178 * t3231 - F::new(4.0) / F::new(3.0) * t1302 * t584 + F::new(4.0) * t5181 * t16);
    t16016
}
