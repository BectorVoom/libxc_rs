//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 882/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk882<F: Float>(t23140: F, t1887: F, t206: F, t22715: F, t242: F, t6612: F, t812: F, t234: F, t852: F, t117: F, t229: F, t67: F) -> (F, F, F, F, F) {
    let t23141 = F::cast_from(0.11304371706359309439e-1_f64) * t23140;
    let t23143 = t22715 * t206 * t1887;
    let t23144 = F::new(35.0) / F::new(432.0) * t23143;
    let t23145 = t6612 * t242;
    let t23146 = t812 * t23145;
    let t23153 = t234 * t852;
    let t23163 = t229 * t67 * t117;
    (t23141, t23144, t23146, t23153, t23163)
}
