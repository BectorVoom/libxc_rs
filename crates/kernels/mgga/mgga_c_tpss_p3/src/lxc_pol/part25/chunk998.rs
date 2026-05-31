//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 998/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk998<F: Float>(t10137: F, t5373: F, t3240: F, t5377: F, t1206: F, t5372: F, t762: F, t1629: F, t4397: F, t5376: F, t10078: F, t10104: F, t10141: F, t1244: F, t12902: F, t13756: F, t13760: F, t13765: F, t13768: F, t13771: F, t3244: F, t3271: F, t4413: F) -> F {
    let t13774 = t10137 * t5373;
    let t13776 = t3240 * t5377;
    let t13780 = t762 * t5372 * t1206;
    let t13784 = t762 * t1629 * t4397;
    let t13788 = t762 * t5376 * t1206;
    let t13791 = t3271 * t13756 / F::cast_from(384.0_f64) - t4413 * t13760 / F::cast_from(384.0_f64) + t4413 * t13765 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t13768 - t1244 * t13771 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t13774 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13776 + t12902 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t10078 - t10104 - t10141 * t13780 / F::cast_from(4.0_f64) + t3244 * t13784 / F::cast_from(8.0_f64) + t3244 * t13788 / F::cast_from(16.0_f64);
    t13791
}
