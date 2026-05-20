//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2791/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2791<F: Float>(t46445: F, t2517: F, t2658: F, t5392: F, t47160: F, t41291: F, t16634: F, t2427: F, t47163: F, t47165: F, t12923: F, t3966: F, t4194: F) -> (F, F, F, F, F, F, F, F) {
    let t59011 = F::new(24.0) * t46445;
    let t59013 = t2658 * t2517 * t5392;
    let t59014 = F::new(12.0) * t59013;
    let t59015 = F::new(2.0) * t47160;
    let t59016 = F::new(8.0) * t41291;
    let t59018 = F::new(8.0) * t2427 * t16634;
    let t59019 = F::new(16.0) * t47163;
    let t59020 = F::new(16.0) * t47165;
    let t59022 = t4194 * t12923 * t3966;
    (t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59022)
}
