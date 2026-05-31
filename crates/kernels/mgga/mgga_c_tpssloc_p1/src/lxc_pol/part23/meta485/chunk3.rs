//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1487/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487<F: Float>(t5464: F, t5488: F, t5468: F, t5396: F, t5480: F, t5484: F, t75910: F, t100: F, t103: F, t104: F, t1447: F, t1450: F, t19488: F, t19513: F, t20245: F, t20318: F, t20322: F, t20332: F, t20335: F, t20338: F, t20339: F, t2341: F, t2349: F, t4049: F, t4059: F, t45460: F, t45496: F, t5475: F, t5481: F, t5485: F, t92: F, t95: F, tau1: F) -> (F, F, F) {
    let t79748 = t5464 * t5464;
    let t79755 = t5488 * t5488;
    let t79761 = t5468 * t5468;
    let t79768 = t5396 * t5396;
    let t79781 = t5480 * t5480;
    let t79788 = t5484 * t5484;
    let t79795 = F::cast_from(12.0_f64) * t75910;
    let t79812 = F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t92 * t45496 * t79761 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t92 * t19488 * t5396 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92 * t2341 * t79768 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t92 * t4049 * t20318 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t5475 * t5481 + F::cast_from(200.0_f64) / F::cast_from(81.0_f64) * t1447 * t20332 - F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t1447 * t20335 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t100 * t45460 * t79781 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t100 * t19513 * t5484 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t100 * t2349 * t79788 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t100 * t4059 * t20338 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t95 * t79795 + F::cast_from(6160.0_f64) / F::cast_from(81.0_f64) * tau1 * t20245 * t104 - F::cast_from(8800.0_f64) / F::cast_from(81.0_f64) * t20322 * t1450 + F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t5475 * t5485 - F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t1447 * t20339 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t103 * t79795;
    (t79748, t79755, t79812)
}
