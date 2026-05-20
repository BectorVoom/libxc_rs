//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1498/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498<F: Float>(t25: F, t17: F, t184: F, t79888: F, t57208: F, t6463: F, t57211: F, t54451: F, t74496: F, t1298: F, t19606: F, t20216: F, t3704: F, t39861: F, t5170: F, t5397: F, t75911: F, t79859: F, t79864: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t79942 = t17 * t79888 * t184;
    let t79946 = F::new(48.0) * t57208;
    let t79947 = t6463 * t6463;
    let t79952 = F::cast_from(0.14649157844805236043e-2_f64) * t57211;
    let t79953 = F::cast_from(0.4155806185363551302e3_f64) * t54451;
    let t79954 = F::new(4.0) * t74496;
    let t79970 = piecewise3::<F>(t26, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39861 * t79859 + F::new(16.0) / F::new(9.0) * t19606 * t5397 - F::new(2.0) / F::new(3.0) * t3704 * t79864 - F::new(8.0) / F::new(9.0) * t5170 * t20216 + F::new(2.0) / F::new(3.0) * t1298 * t75911);
    (t79942, t79946, t79947, t79952, t79953, t79954, t79970)
}
