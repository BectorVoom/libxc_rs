//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 850/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk850<F: Float>(t2697: F, t5624: F, t13360: F, t1516: F, t5568: F, t9573: F, t2563: F, t5572: F, t5576: F, t838: F, t5631: F, t814: F, t252: F, t5611: F, t1499: F, t4280: F) -> (F, F, F, F, F, F, F, F) {
    let t16988 = t2697 * t5624;
    let t16990 = t13360 * t1516;
    let t16993 = t9573 * t5568;
    let t16995 = t2563 * t5572;
    let t17000 = t5576 * t838;
    let t17027 = t814 * t5631;
    let t17030 = t252 * t5611;
    let t17034 = t1499 * t4280;
    (t16988, t16990, t16993, t16995, t17000, t17027, t17030, t17034)
}
