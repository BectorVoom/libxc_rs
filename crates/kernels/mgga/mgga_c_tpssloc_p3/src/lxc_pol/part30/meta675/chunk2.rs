//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2106/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2106<F: Float>(t27960: F, t645: F, t72: F, t4021: F, t7431: F, t1864: F, t5389: F, t1863: F, t22544: F, t26009: F, t26013: F, t26016: F, t27937: F, t33567: F, t6506: F, t6510: F, t83717: F, t83830: F, t90087: F, t90091: F, t90095: F, t90098: F, t90101: F, t90104: F, t9239: F) -> F {
    let t96418 = t72 * t27960 * t645;
    let t96422 = t72 * t7431 * t4021;
    let t96425 = t1864 * t5389;
    let t96426 = t1863 * t96425;
    let t96441 = F::new(20.0) * t9239 * t33567 * t26009 - t27937 * t6506 / F::new(6.0) - t27937 * t6510 / F::new(6.0) + F::new(35.0) * t83830 * t96418 - F::new(10.0) * t22544 * t96422 + F::new(10.0) * t83717 * t96426 - F::new(10.0) / F::new(3.0) * t90098 * t26013 - F::new(10.0) / F::new(3.0) * t90101 * t26013 - F::new(10.0) / F::new(3.0) * t90104 * t26013 - F::new(10.0) / F::new(3.0) * t26016 * t90087 - F::new(10.0) / F::new(3.0) * t26016 * t90091 - F::new(10.0) / F::new(3.0) * t26016 * t90095;
    t96441
}
