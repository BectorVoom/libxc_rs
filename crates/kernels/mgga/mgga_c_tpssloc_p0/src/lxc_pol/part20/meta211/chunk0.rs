//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1251/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1251<F: Float>(t25: F, t28: F, t17: F, t5168: F, t1408: F, t3704: F, t1298: F, t2: F, t584: F, t606: F, t1649: F, t3711: F, t1302: F, t1081: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t5169 = t17 * t5168;
    let t5170 = t3704 * t1408;
    let t5173 = t1298 * t2;
    let t5177 = piecewise3::<F>(t26, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5170 * t606 + F::new(4.0) / F::new(3.0) * t5173 * t584);
    let t5178 = t3711 * t1649;
    let t5181 = t1302 * t2;
    let t5185 = piecewise3::<F>(t29, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5178 * t1081 - F::new(4.0) / F::new(3.0) * t5181 * t584);
    (t5169, t5170, t5173, t5177, t5178, t5181, t5185)
}
