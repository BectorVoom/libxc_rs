//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1190/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1190<F: Float>(t81281: F, t12023: F, t12033: F, t1375: F, t1385: F, t2092: F, t24138: F, t24139: F, t24147: F, t3758: F, t3887: F, t39916: F, t7194: F, t7214: F, t81264: F, t81267: F, t81272: F, t81284: F) -> F {
    let t84423 = F::cast_from(0.19739208802178717238e0_f64) * t81281;
    let t84429 = -F::new(3.0) * t3758 * t24139 - F::new(6.0) * t7194 * t12023 + F::cast_from(0.15626873635058151147e0_f64) * t81264 - F::new(3.0) * t12033 * t7214 + F::new(6.0) * t1375 * t3887 * t24138 * t1385 + F::cast_from(0.49348022005446793095e-1_f64) * t81267 - F::cast_from(0.19739208802178717238e0_f64) * t81272 + t84423 + F::new(12.0) * t3758 * t24147 - F::new(3.0) * t39916 * t2092 + F::cast_from(0.9869604401089358619e-1_f64) * t81284;
    t84429
}
