//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2019/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2019<F: Float>(t91135: F, t91137: F, t91140: F, t91149: F, t91154: F, t91158: F, t91161: F, t91167: F, t91170: F, t91133: F, t91143: F, t91145: F, t91147: F, t91163: F, t91165: F, t91173: F, t91176: F, t91179: F) -> F {
    let t93644 = F::new(7.0) / F::new(144.0) * t91135;
    let t93645 = F::new(7.0) / F::new(144.0) * t91137;
    let t93646 = F::cast_from(0.80745512188280781706e-3_f64) * t91140;
    let t93650 = F::new(119.0) / F::new(864.0) * t91149;
    let t93651 = F::cast_from(0.13457585364713463618e-3_f64) * t91154;
    let t93652 = F::cast_from(0.26915170729426927236e-3_f64) * t91158;
    let t93653 = F::new(7.0) / F::new(144.0) * t91161;
    let t93656 = F::cast_from(0.22608743412718618878e-1_f64) * t91167;
    let t93657 = F::new(7.0) / F::new(12.0) * t91170;
    let t93661 = F::new(5.0) / F::new(192.0) * t91133 + t93644 + t93645 - t93646 - F::cast_from(0.80745512188280781706e-3_f64) * t91143 - t91145 / F::new(96.0) - t91147 / F::new(192.0) - t93650 + t93651 - t93652 + t93653 - t91163 / F::new(192.0) - t91165 / F::new(192.0) - t93656 - t93657 + t91173 / F::new(4.0) + t91176 / F::new(8.0) - F::cast_from(0.23739180583354549822e0_f64) * t91179;
    t93661
}
