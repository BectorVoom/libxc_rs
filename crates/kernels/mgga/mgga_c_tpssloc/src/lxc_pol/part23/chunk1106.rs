//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1106/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1106<F: Float>(t2517: F, t5520: F, t12945: F, t4205: F, t32: F, t5519: F, t5398: F, t707: F, t16616: F, t2535: F, t2371: F, t41115: F, t5593: F, t5572: F, t9541: F, t5624: F, t9601: F) -> (F, F, F, F, F, F, F, F, F) {
    let t57897 = t5520 * t2517;
    let t57960 = t4205 * t12945;
    let t57973 = t32 * t5519;
    let t57992 = t707 * t2517 * t5398;
    let t58021 = t16616 * t2535;
    let t58057 = t16616 * t2371;
    let t58421 = t41115 * t5593;
    let t58550 = t9541 * t5572;
    let t58574 = t9601 * t5624;
    (t57897, t57960, t57973, t57992, t58021, t58057, t58421, t58550, t58574)
}
