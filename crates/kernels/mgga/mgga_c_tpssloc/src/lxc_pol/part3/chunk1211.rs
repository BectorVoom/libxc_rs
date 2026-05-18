//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1211/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1211<F: Float>(t25: F, t12061: F, t1408: F, t2: F, t3664: F, t584: F, t606: F, t16: F, t2249: F, t3665: F, t5134: F, t5137: F, t514: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t15937 = t12061 * t1408;
    let t15940 = t3664 * t2;
    let t15941 = t584 * t606;
    let t15951 = piecewise3::<f64>(t26, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15937 * t3665 + F::new(16.0) / F::new(9.0) * t15940 * t15941 + F::new(4.0) / F::new(9.0) * t5134 * t2249 + F::new(8.0) / F::new(3.0) * t514 * t584 - F::new(8.0) * t5137 * t16);
    (t15941, t15951)
}
