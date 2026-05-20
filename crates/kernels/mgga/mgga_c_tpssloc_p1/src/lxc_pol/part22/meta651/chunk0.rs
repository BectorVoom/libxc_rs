//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2192/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192<F: Float>(t4194: F, t5398: F, t607: F, t750: F, t32: F, t5519: F, t2517: F, t707: F, t16616: F, t2535: F, t16701: F, t2427: F) -> (F, F, F, F, F) {
    let t57965 = t4194 * t750 * t5398 * t607;
    let t57973 = t32 * t5519;
    let t57992 = t707 * t2517 * t5398;
    let t58021 = t16616 * t2535;
    let t58047 = t2427 * t16701;
    (t57965, t57973, t57992, t58021, t58047)
}
