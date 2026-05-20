//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1013/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1013<F: Float>(t27480: F, t27529: F, t27568: F, t27739: F, t1241: F, t2154: F, t5088: F, t3598: F, t1751: F, t7299: F, t7302: F, t24574: F, t8015: F) -> (F, F, F, F, F, F, F) {
    let t27741 = t27480 + t27529 + t27568 + t27739;
    let t27742 = t1241 * t27741;
    let t27746 = t2154 * t5088;
    let t27747 = t3598 * t27746;
    let t27751 = t7299 * t1751;
    let t27752 = t27751 * t7302;
    let t27755 = t24574 * t8015;
    (t27741, t27742, t27746, t27747, t27751, t27752, t27755)
}
