//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 985/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk985<F: Float>(t3240: F, t5377: F, t1206: F, t5372: F, t762: F, t1629: F, t4397: F, t5376: F, t10078: F, t10104: F, t10141: F, t1244: F, t12902: F, t13756: F, t13760: F, t13765: F, t13768: F, t13771: F, t13774: F, t3244: F, t3271: F, t4413: F) -> (F,) {
    let t13776 = t3240 * t5377;
    let t13780 = t762 * t5372 * t1206;
    let t13784 = t762 * t1629 * t4397;
    let t13788 = t762 * t5376 * t1206;
    let t13791 = t3271 * t13756 / 384.0 - t4413 * t13760 / 384.0 + t4413 * t13765 / 768.0 + 7.0 / 1152.0 * t13768 - t1244 * t13771 / 768.0 - 7.0 / 48.0 * t13774 + 7.0 / 144.0 * t13776 + t12902 - 119.0 / 13824.0 * t10078 - t10104 - t10141 * t13780 / 4.0 + t3244 * t13784 / 8.0 + t3244 * t13788 / 16.0;
    (t13791,)
}
