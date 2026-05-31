//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1267/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1267<F: Float>(t30720: F, t849: F, t23083: F, t30706: F, t23094: F, t30703: F, t23103: F, t794: F, t8339: F, t30719: F, t808: F, t8344: F) -> (F, F, F, F, F) {
    let t112820 = t30720 * t849;
    let t112821 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t112820;
    let t112829 = t23083 * t30706;
    let t112830 = F::cast_from(0.11304371706359309439e-1_f64) * t112829;
    let t112834 = t23094 * t30703;
    let t112840 = t23103 * t794 * t8339;
    let t112846 = t808 * t30719 * t8344;
    (t112821, t112830, t112834, t112840, t112846)
}
