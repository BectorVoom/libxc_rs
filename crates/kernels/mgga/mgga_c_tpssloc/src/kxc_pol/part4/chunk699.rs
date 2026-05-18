//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 699/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk699<F: Float>(t1118: F, t4781: F, t1099: F, t1670: F, t3315: F, t1117: F, t3313: F, t3238: F, t3319: F, t4721: F, t4726: F, t4731: F, t4735: F) -> (F, F, F, F, F, F) {
    let t4782 = t4781 * t1118;
    let t4784 = F::new(1.0) * t1099 * t4782;
    let t4785 = t1670 * t3315;
    let t4786 = t4785 * t1117;
    let t4788 = F::new(0.16081979498692535067e2) * t3313 * t4786;
    let t4794 = t3319 - F::new(0.57077777777777777777e-2) * t3238 - F::new(0.57077777777777777777e-2) * t4721 - F::new(0.11415555555555555555e-1) * t4726 + F::new(0.34246666666666666666e-1) * t4731 + F::new(0.17123333333333333333e-1) * t4735;
    (t4782, t4784, t4785, t4786, t4788, t4794)
}
