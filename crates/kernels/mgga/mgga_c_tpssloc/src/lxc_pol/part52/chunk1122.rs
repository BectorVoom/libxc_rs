//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1122/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1122<F: Float>(t23083: F, t30706: F, t23094: F, t30703: F, t23103: F, t794: F, t8339: F, t30719: F, t808: F, t8344: F, t226: F, t235: F, t2690: F, t23139: F, t23171: F, t23228: F, t8335: F) -> (F, F, F, F, F, F, F) {
    let t112829 = t23083 * t30706;
    let t112834 = t23094 * t30703;
    let t112835 = 0.21083550404717759669e-2 * t112834;
    let t112840 = t23103 * t794 * t8339;
    let t112841 = 0.6728792682356731809e-4 * t112840;
    let t112846 = t808 * t30719 * t8344;
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112851 = 119.0 / 6912.0 * t112850;
    let t112855 = t23139 * t8339;
    let t112856 = 0.45217486825437237757e-1 * t112855;
    let t112863 = 0.16449340668482264365e-1 * t23171 * t23228 * t8335;
    (t112829, t112835, t112841, t112846, t112851, t112856, t112863)
}
