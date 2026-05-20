//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2245/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2245<F: Float>(t25115: F, t7496: F, t87451: F, t23133: F, t5628: F, t23041: F, t5614: F, t1512: F, t87261: F, t81850: F, t81853: F, t87292: F, t87293: F, t87301: F, t87306: F, t92633: F, t98715: F, t98717: F, t98719: F, t98721: F, t98723: F, t98725: F, t98728: F) -> F {
    let t98731 = t87451 * t7496 * t25115;
    let t98733 = t23133 * t5628;
    let t98736 = t23041 * t5614;
    let t98738 = t87261 * t1512;
    let t98740 = -t81850 - t81853 + t87292 + F::cast_from(0.16956557559538964159e-1_f64) * t87293 - t87301 + F::new(5.0) / F::new(192.0) * t98715 - F::new(5.0) / F::new(64.0) * t98717 + F::new(5.0) / F::new(192.0) * t98719 + F::new(5.0) / F::new(384.0) * t98721 - t98723 / F::new(1536.0) + F::cast_from(0.14130464632949136799e-2_f64) * t98725 - F::cast_from(0.48447307312968469024e-2_f64) * t98728 + F::cast_from(0.24223653656484234512e-2_f64) * t98731 + F::new(7.0) / F::new(576.0) * t98733 - t92633 - F::cast_from(0.13565246047631171327e0_f64) * t87306 + F::new(7.0) / F::new(2304.0) * t98736 + F::new(7.0) / F::new(1152.0) * t98738;
    t98740
}
