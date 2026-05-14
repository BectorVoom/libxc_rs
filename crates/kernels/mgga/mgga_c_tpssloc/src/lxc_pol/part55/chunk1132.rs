//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1132/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1132<F: Float>(t125424: F, t125459: F, t125482: F, t125508: F, t24574: F, t34285: F, t118136: F, t118173: F, t1186: F, t24745: F, t24849: F, t27453: F, t27477: F, t27525: F, t32457: F, t32475: F, t34295: F, t34300: F, t34301: F, t3604: F, t3624: F, t470: F, t4733: F, t493: F, t5064: F, t5079: F, t7283: F, t7362: F, t7373: F, t7375: F, t7376: F) -> (F, F) {
    let t125510 = t125424 + t125459 + t125482 + t125508;
    let t125523 = t24574 * t34285;
    let t125530 = t3604 * t34301 + 0.16449340668482264365e-1 * t7373 * t7375 * t27477 * t7376 - 0.16449340668482264365e-1 * t7283 * t1186 * t34295 + t470 * t493 * t125510 - 0.18277045187202515961e-2 * t118173 - 0.16449340668482264365e-1 * t7283 * t27453 * t24745 * t32457 - t3624 * t34300 * t5079 - 0.54831135561607547883e-2 * t24849 * t118136 * t27525 - 0.18277045187202515961e-2 * t125523 - 0.54831135561607547883e-2 * t7283 * t7362 * t32457 * t4733 + t5064 * t32475;
    (t125510, t125530)
}
