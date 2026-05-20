//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2447/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2447<F: Float>(t135: F, t21458: F, t973: F, t20234: F, t42841: F, t2986: F, t4514: F, t61189: F, t10186: F, t10235: F, t13798: F, t17863: F, t21433: F, t21459: F, t21476: F, t2960: F, t42811: F, t42817: F, t4510: F, t48217: F, t61074: F, t61172: F, t61210: F, t68462: F, t68466: F, t68470: F, t68481: F, t68521: F) -> F {
    let t69540 = t973 * t135 * t21458;
    let t69548 = t42841 * t20234;
    let t69570 = t2986 * t61189 * t4514;
    let t69574 = F::cast_from(0.13333333333333333332e-1_f64) * t2986 * t4510 * t68481 + F::cast_from(0.22222222222222222222e-2_f64) * t2960 * t21459 - F::cast_from(0.27777777777777777777e-3_f64) * t69540 - F::cast_from(0.66666666666666666663e-2_f64) * t2986 * t4510 * t68462 - F::cast_from(0.1037037037037037037e-1_f64) * t2986 * t13798 * t68521 + F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t10235 * t69548 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4510 * t68466 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4510 * t68470 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t48217 * t17863 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t61210 * t4514 + F::cast_from(0.22222222222222222222e-2_f64) * t61074 + F::cast_from(0.44444444444444444443e-2_f64) * t10186 * t21476 + F::cast_from(0.22222222222222222222e-2_f64) * t10186 * t21433 - F::cast_from(0.27777777777777777777e-3_f64) * t69570 - F::cast_from(0.82304526748971193413e-3_f64) * t42811 - t42817 - F::cast_from(0.83333333333333333331e-3_f64) * t61172;
    t69574
}
