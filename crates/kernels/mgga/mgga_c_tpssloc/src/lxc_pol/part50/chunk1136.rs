//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1136/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1136<F: Float>(t23083: F, t30706: F, t23094: F, t30703: F, t23103: F, t794: F, t8339: F, t30719: F, t808: F, t8344: F, t226: F, t235: F, t2690: F) -> (F, F, F, F, F) {
    let t112829 = t23083 * t30706;
    let t112834 = t23094 * t30703;
    let t112835 = F::new(0.21083550404717759669e-2) * t112834;
    let t112840 = t23103 * t794 * t8339;
    let t112841 = F::new(0.6728792682356731809e-4) * t112840;
    let t112846 = t808 * t30719 * t8344;
    let t112850 = t226 * t235 * t2690 * t8344;
    (t112829, t112835, t112841, t112846, t112850)
}
