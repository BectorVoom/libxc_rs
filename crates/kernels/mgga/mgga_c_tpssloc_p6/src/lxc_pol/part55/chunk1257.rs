//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1257/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1257<F: Float>(t75795: F, t8319: F, t1395: F, t1458: F, t25994: F, t7266: F, t652: F, t6534: F, t8103: F, t26168: F, t8690: F, t33746: F, t6880: F) -> (F, F, F, F, F, F) {
    let t120848 = F::cast_from(27.0_f64) * t75795 * t8319;
    let t120849 = t1395 * t1458;
    let t120851 = F::cast_from(27.0_f64) * t120849 * t8319;
    let t122875 = t7266 * t25994;
    let t122897 = t652 * t8103 * t6534;
    let t122910 = t8690 * t26168;
    let t122914 = t33746 * t6880;
    (t120848, t120851, t122875, t122897, t122910, t122914)
}
