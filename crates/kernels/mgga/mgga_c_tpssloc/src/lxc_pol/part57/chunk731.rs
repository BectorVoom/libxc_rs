//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 731/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk731<F: Float>(t1458: F, t23880: F, t26523: F, t28868: F, t28888: F, t28890: F, t28892: F, t28895: F, t28898: F, t28901: F, t28903: F, t5456: F, t5493: F, t577: F, t7010: F, t2031: F, t27956: F) -> (F, F) {
    let t28904 = 0.45e1 * t28868 * t577 + 27.0 * t26523 * t1458 + 27.0 * t23880 * t5456 + 0.135e2 * t7010 * t5493 + t28888 + t28890 + t28892 + t28895 + t28898 + t28901 + t28903;
    let t28935 = t2031 * t27956;
    (t28904, t28935)
}
