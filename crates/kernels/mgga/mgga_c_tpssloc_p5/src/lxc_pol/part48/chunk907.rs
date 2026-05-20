//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 907/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk907<F: Float>(t22690: F, t23122: F, t6619: F, t776: F, t30720: F, t849: F, t2707: F, t8343: F, t2703: F, t30709: F, t23083: F, t30706: F) -> (F, F, F, F, F, F) {
    let t112818 = t23122 * t22690 * t6619 * t776;
    let t112820 = t30720 * t849;
    let t112823 = t8343 * t2707;
    let t112825 = t8343 * t2703;
    let t112827 = t30709 * t849;
    let t112829 = t23083 * t30706;
    (t112818, t112820, t112823, t112825, t112827, t112829)
}
