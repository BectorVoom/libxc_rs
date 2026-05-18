//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1129/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1129<F: Float>(t34210: F, t34384: F, t3: F, t1458: F, t32643: F, t33184: F, t33187: F, t33190: F, t33192: F, t33195: F, t33774: F, t33776: F, t33778: F, t577: F, t8508: F) -> (F, F, F) {
    let t34385 = t34210 + t34384;
    let t34386 = t3 * t34385;
    let t34401 = F::new(0.45e1) * t34385 * t577 + F::new(0.135e2) * t32643 * t1458 + F::new(27.0) * t33774 + F::new(54.0) * t33776 + F::new(27.0) * t33778 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t34385, t34386, t34401)
}
